### Неустранимые / unrecoverable ошибки

например:
- обращение к массиву за его пределами
- panic!

по умолчанию при панике
- вывод сообщения
- раскрутка и очистка стека вызовов / unwind, clean up
- завершение работы

```toml
Cargo.toml
[profile.release]
panic = 'abort'
```

при панике программа немедленно завершится
- очистка памяти в зоне ответственности OS
- исполняемый файл меньше

```bash
RUST_BACKTRACE=1 cargo run
```
для получения обратной трассировки
