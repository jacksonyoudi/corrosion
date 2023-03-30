# Docs

### 列出3个常用的宏、3个常用的存储数据结构
说明：使用 Substrate-node-template 的版本为 polkadot-v0.9.30（git clone -b polkadot-v0.9.30 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git）


#### 3个常用的宏
decl_module! - 用于定义 Substrate 的 runtime 模块，包括模块的名称、存储项、事件和可调用函数等。
```rust
  decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // 模块定义
    }
}
```

decl_storage! - 用于定义 Substrate 的存储项，包括存储项的名称、类型和默认值等。

```rust
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // 存储项定义
    }
}
```

decl_event! - 用于定义 Substrate 的事件，包括事件的名称和参数等。
```rust
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        // 事件定义
    }
);
```

#### 3个常用的存储数据结构
StorageValue - 存储单个值，可以通过 get 和 set 方法进行读写操作。

```rust
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        MyValue: u32;
    }
}

fn my_function<T: Trait>() -> u32 {
    let my_value = MyValue::<T>::get();
    MyValue::<T>::put(my_value + 1);
    my_value
}
```

StorageMap - 存储键值对，可以通过 get、insert 和 remove 等方法进行读写操作。
```rust
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        MyMap: map hasher(blake2_128_concat) u32 => u32;
    }
}

fn my_function<T: Trait>(key: u32, value: u32) {
    MyMap::<T>::insert(key, value);
    let my_value = MyMap::<T>::get(key);
    MyMap::<T>::remove(key);
}
```
StorageDoubleMap - 存储双重键值对，可以通过 get、insert 和 remove 等方法进行读写操作。
```rust
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        MyDoubleMap: double_map hasher(blake2_128_concat) u32, hasher(blake2_128_concat) u32 => u32;
    }
}

fn my_function<T: Trait>(key1: u32, key2: u32, value: u32) {
    MyDoubleMap::<T>::insert(key1, key2, value);
    let my_value = MyDoubleMap::<T>::get(key1, key2);
    MyDoubleMap::<T>::remove(key1, key2);
}
```





