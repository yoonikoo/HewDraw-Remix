#[skyline::hook(offset = 0xb35edc, inline)]
unsafe fn jack_null_regular_damage(ctx: &skyline::hooks::InlineCtx) {
    std::arch::asm!("fmov s8, 0.0");
}

pub fn install() {
    skyline::install_hook!(jack_null_regular_damage);
}
