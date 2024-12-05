# หลักการ 4 ประการของ OOP ใน Rust 🦀

## 1. การห่อหุ้ม (Encapsulation)

ใน Rust เราสามารถซ่อนข้อมูลและการทำงานภายในได้โดยใช้ระบบ Privacy:

- `pub` - เข้าถึงได้จากภายนอก module
- ไม่ระบุ - เข้าถึงได้เฉพาะภายใน module เท่านั้น

```rust
pub struct User {
    name: String,      // private
    pub age: u32      // public
}
```

## 2. การซ่อนรายละเอียด (Abstraction)

Rust ใช้ Traits เพื่อกำหนดพฤติกรรมที่เป็นนามธรรม:

```rust
pub trait Animal {
    fn make_sound(&self);
    fn eat(&self);
}
```

## 3. การประกอบร่วม (Composition Over Inheritance)

Rust ไม่มี inheritance แต่แนะนำให้ใช้ Composition แทน:

```rust
struct Engine {
    horsepower: u32
}

struct Car {
    engine: Engine,    // Composition
    brand: String
}
```

## 4. การพ้องรูป (Polymorphism)

Rust ใช้ Traits เพื่อทำ Polymorphism:

```rust
trait Drawable {
    fn draw(&self);
}

impl Drawable for Circle {
    fn draw(&self) {
        // วาดวงกลม
    }
}

impl Drawable for Square {
    fn draw(&self) {
        // วาดสี่เหลี่ยม
    }
}
```

## ทำไม Rust ถึงไม่มี Inheritance? 🤔

Rust เลือกที่จะไม่สนับสนุน inheritance ด้วยเหตุผลหลักๆ ดังนี้:

1. **ลดความซับซ้อน** - การสืบทอดมักทำให้โค้ดซับซ้อนและเข้าใจยาก
2. **หลีกเลี่ยงปัญหา Diamond Problem** - ปัญหาที่เกิดจากการสืบทอดหลายชั้น
3. **ส่งเสริมการใช้ Composition** - ทำให้โค้ดยืดหยุ่นและทดสอบง่ายกว่า
4. **ใช้ Traits แทน** - ให้ความยืดหยุ่นในการออกแบบโดยไม่ต้องผูกติดกับลำดับชั้น

> 💡 Rust เน้นการออกแบบที่ปลอดภัยและชัดเจน จึงเลือกใช้ Traits และ Composition แทนการสืบทอด
