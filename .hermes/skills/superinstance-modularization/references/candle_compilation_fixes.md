# Candle 0.11 Compilation Fixes (2026-08-18)

## Common Errors & Fixes

### 1. `Tensor::from_vec` Shape Tuples
**Error**: `expected tuple of 2 elements, found tuple of 3 elements`
**Cause**: `Tensor::from_vec(vec, (batch, seq_len, hidden))` — 3-tuple not supported
**Fix**: Use 2-tuple `(batch * seq_len, hidden)` or `(batch, seq_len * hidden)`
```rust
// Before (breaks)
Tensor::from_vec(data, (batch_size, seq_len, hidden_size), &device)

// After (works)
let shape = (batch_size * seq_len, hidden_size);
Tensor::from_vec(data, shape, &device)
```

### 2. `IndexOp` Trait for `.i()` Indexing
**Error**: `no method named 'i' found for struct 'Tensor'`
**Fix**: Import the trait
```rust
use candle_core::{Tensor, IndexOp};  // Add IndexOp
let token = input_ids.i(0)?;  // Now works
```

### 3. `BertModel::forward` Signature
**Error**: `expected 2 arguments, found 3` or type mismatch
**Fix**: Second argument is `Option<&Tensor>` for attention_mask
```rust
// Before (breaks)
let output = model.forward(&input_ids, &attention_mask)?;

// After (works)
let output = model.forward(&input_ids, Some(&attention_mask))?;
```

### 4. `cls_pooling` Return Type
**Error**: `expected enum 'Result', found struct 'Tensor'`
**Fix**: Handle the Result
```rust
// Before (breaks)
let pooled = cls_pooling(&output, &attention_mask)?;

// After (works)
let pooled = cls_pooling(&output, &attention_mask)?;
```

### 5. `max_len` Calculation Chain
**Error**: `no method named 'min' found for type 'usize'` or Option handling
**Fix**: Use `.min(max_len).unwrap_or(256)` pattern
```rust
let max_len = attention_mask.sum_all()?.to_scalar::<u32>()? as usize;
let max_len = max_len.min(config.max_length).unwrap_or(256);
```

### 6. `Error as CandleError` Import
**Fix**: Import explicitly for error handling
```rust
use candle_core::{Device, Tensor, DType, IndexOp, Error as CandleError};
```

## Model Loading Pattern (MiniLM/MPNet/BGE/E5)
```rust
let device = if config.use_cuda && candle_core::cuda_is_available() {
    Device::new_cuda(config.cuda_device_id)?
} else {
    Device::Cpu
};

let tokenizer = Tokenizer::from_file(tokenizer_path)
    .map_err(|e| EmbedderError::Tokenization(e.to_string()))?;

let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
let model = BertModel::load(vb, &config)?;
```

## Pooling Strategies
```rust
fn cls_pooling(output: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    let cls = output.i(0)?.i(0)?;  // [batch, seq, hidden] -> [batch, hidden]
    Ok(cls)
}

fn mean_pooling(output: &Tensor, attention_mask: &Tensor) -> Result<Tensor> {
    let mask = attention_mask.unsqueeze(2)?;  // [batch, seq, 1]
    let masked = output.broadcast_mul(&mask)?;  // [batch, seq, hidden]
    let summed = masked.sum(1)?;  // [batch, hidden]
    let counts = mask.sum(1)?;  // [batch, 1]
    let pooled = summed.broadcast_div(&counts)?;  // [batch, hidden]
    Ok(pooled)
}
```

## Device Selection with Fallback
```rust
fn get_device(use_cuda: bool, device_id: usize, fallback: bool) -> Result<Device> {
    if use_cuda {
        match Device::new_cuda(device_id) {
            Ok(d) => Ok(d),
            Err(e) if fallback => {
                eprintln!("CUDA unavailable ({}), falling back to CPU", e);
                Ok(Device::Cpu)
            }
            Err(e) => Err(e.into()),
        }
    } else {
        Ok(Device::Cpu)
    }
}
```