import cv2, threading, time, json, sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

LOG = open(r"C:\Users\casey\proart-cam.log", "a", buffering=1)
def log(*a):
    print(time.strftime("%H:%M:%S"), *a, file=LOG)
    print(time.strftime("%H:%M:%S"), *a)

state = {"frame": None, "jpg": None, "ts": 0.0, "count": 0, "shape": None, "cam": -1}

def pick_camera():
    best = None
    for idx in range(5):
        cap = cv2.VideoCapture(idx, cv2.CAP_DSHOW)
        if not cap.isOpened():
            cap.release(); continue
        cap.set(cv2.CAP_PROP_FRAME_WIDTH, 1280)
        cap.set(cv2.CAP_PROP_FRAME_HEIGHT, 720)
        ok, fr = cap.read()
        if ok and fr is not None:
            h, w = fr.shape[:2]
            mean = float(fr.mean())
            log(f"cam {idx}: {w}x{h} mean={mean:.1f}")
            score = w * h
            if best is None or score > best[0]:
                if best: best[1].release()
                best = (score, cap, idx, (w, h))
                continue
        cap.release()
    if best:
        log(f"selected cam {best[2]} at {best[3]}")
        return best[1], best[2]
    return None, -1

def capture_loop():
    cap, idx = pick_camera()
    if cap is None:
        log("FATAL: no camera opened"); return
    state["cam"] = idx
    while True:
        ok, fr = cap.read()
        if not ok:
            log("read failed, retrying"); time.sleep(0.5); continue
        ok2, buf = cv2.imencode(".jpg", fr, [cv2.IMWRITE_JPEG_QUALITY, 82])
        if not ok2: continue
        state["frame"] = fr; state["jpg"] = buf.tobytes()
        state["ts"] = time.time(); state["count"] += 1
        state["shape"] = fr.shape
        if state["count"] % 300 == 1:
            log(f"frames={state['count']} shape={fr.shape} mean={float(fr.mean()):.1f}")

class H(BaseHTTPRequestHandler):
    def log_message(self, *a): pass
    def do_GET(self):
        if self.path.startswith("/shot"):
            if state["jpg"] is None:
                self.send_response(503); self.end_headers(); return
            self.send_response(200)
            self.send_header("Content-Type", "image/jpeg")
            self.send_header("Content-Length", str(len(state["jpg"])))
            self.end_headers(); self.wfile.write(state["jpg"])
        elif self.path.startswith("/stream") or self.path.startswith("/proart"):
            self.send_response(200)
            self.send_header("Content-Type", "multipart/x-mixed-replace; boundary=frame")
            self.end_headers()
            last = 0
            try:
                while True:
                    if state["ts"] != last and state["jpg"] is not None:
                        last = state["ts"]
                        self.wfile.write(b"--frame\r\nContent-Type: image/jpeg\r\n\r\n" + state["jpg"] + b"\r\n")
                    time.sleep(0.1)
            except (BrokenPipeError, ConnectionResetError):
                pass
        else:
            body = json.dumps({"cam": state["cam"], "frames": state["count"],
                               "age_s": round(time.time() - state["ts"], 2) if state["ts"] else None,
                               "shape": str(state["shape"])}).encode()
            self.send_response(200); self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body))); self.end_headers(); self.wfile.write(body)

if __name__ == "__main__":
    log("proart-cam starting")
    threading.Thread(target=capture_loop, daemon=True).start()
    ThreadingHTTPServer(("0.0.0.0", 8090), H).serve_forever()
