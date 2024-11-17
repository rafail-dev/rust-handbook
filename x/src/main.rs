fn foo_u8(v: u8) -> u8 {
    v
}

fn foo_u16(v: u16) -> u16 {
    v
}

fn main() {
    {
        let v = 0;
        foo_u8(v);
        foo_u16(v.into());
    }

    {
        let v = 0;
        foo_u16(v.into());
        foo_u8(v);
    }

    {
        let v: u8 = 0;
        foo_u16(v.into());
    }

    {
        let v: u16 = 0;
        // the trait bound `u8: From<u16>` is not satisfied
        // foo_u8(v.into());
    }
}
