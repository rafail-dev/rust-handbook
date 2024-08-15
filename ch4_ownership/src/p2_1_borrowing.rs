pub fn main() {
    {
        let _s = String::from("hello");

        // передача ссылки на _s1
        // s (внутри функции) ссылается на _s1,
        // а _s1 ссылается на heap
        let _length = take_only_reference(&_s);

        // _s1 доступна
        // процесс создания ссылки - заимствование (borrowing)

        // expected &String, found String
        // let _length = take_only_reference(_s1);

        // * - обратный & оператор "разыменования"

        // нельзя передать mut ссылку, т.к.
        // значение создано как иммутабельное
        // cannot borrow as mutable
        // mutate(&mut _s);
    }

    {
        let mut _s = String::from("hello");

        mutate(&mut _s);

        // можно использовать иммутабельную ссылку
        // для мутабельного значения (но нельзя наоборот)
        let _length = take_only_reference(&_s);

        // hello!
        // println!("{}", _s);
    }

    {
        let mut _s = String::from("hello");

        // кол-во иммутабельных ссылок может быть любым
        // но нельзя иметь несколько одновременно
        // используемых мутабельных ссылок
        // (а также мутабельную при наличии иммутабельных)

        // В любой момент времени у вас может быть одна (но не обе) изменяемая ссылка
        // или любое количество неизменяемых ссылок.
        let _r1 = &_s;
        let _r2 = &_s;
        let _r3 = &_s;

        // hello, hello, hello
        println!("{}, {}, {}", _r1, _r2, _r3);
        // область действия _r1, _r2, _r3 закончилась
        // поэтому далее можно создать mut ссылку
        //
        // ошибки не было бы, если бы _mr1
        // не использовалась после объявления _mr2
        let _mr1 = &mut _s;
        // cannot borrow `_s` as mutable more than once at a time
        // let _mr2 = &mut _s;

        // hello
        println!("{}", _mr1)
    }

    {
        let mut _s = String::from("hello");

        let _r1 = &_s;

        // cannot borrow `_s` as mutable because
        // it is also borrowed as immutable mutable borrow
        // let  _mr1 = &mut _s;
        // println!("{}, {}", _r1, _mr1)
    }
}

fn take_only_reference(s: &String) -> usize {
    s.len()
}

fn mutate(s: &mut String) -> () {
    s.push('!');
}
