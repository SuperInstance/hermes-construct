import cv2, time, urllib.request

URL = "http://192.168.0.78:8091/push"
LOG = open(r"C:\Users\casey\proart-push.log", "a", buffering=1)
def log(*a): print(time.strftime("%H:%M:%S"), *a, file=LOG)

def main():
    log("proart-push starting ->", URL)
    cap = cv2.VideoCapture(0, cv2.CAP_DSHOW)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, 1280)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, 720)
    if not cap.isOpened():
        log("FATAL camera open failed"); return
    n = 0; errs = 0
    while True:
        ok, fr = cap.read()
        if not ok:
            log("read fail"); time.sleep(0.5); continue
        ok2, buf = cv2.imencode(".jpg", fr, [cv2.IMWRITE_JPEG_QUALITY, 80])
        if not ok2: continue
        try:
            req = urllib.request.Request(URL, data=buf.tobytes(),
                                         headers={"Content-Type": "image/jpeg"}, method="POST")
            urllib.request.urlopen(req, timeout=3).read()
            n += 1; errs = 0
            if n % 240 == 1: log(f"pushed={n} mean={float(fr.mean()):.1f}")
        except Exception as e:
            errs += 1
            if errs <= 3 or errs % 60 == 0: log("push error:", e)
            time.sleep(1.0)
        time.sleep(0.5)

if __name__ == "__main__":
    main()
