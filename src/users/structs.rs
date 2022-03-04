#[derive(FromForm)]
pub struct Password<'r> {
    #[field(name = "password")]
    value: &'r str,
    #[field(validate = eq(self.value))]
    #[field(validate = omits("no"))]
    confirm: &'r str,
}

#[derive(FromForm)]
pub struct User<'r> {
    #[field(name = "password")]
    value: &'r str,
}
