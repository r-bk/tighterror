// Checks that error kinds of more than 64 bits fail to compile with a
// message that names the widest supported underlying type.

type R = <tighterror::Bits<65> as tighterror::Repr>::R;

fn main() {
    // Use the type of 65-bit error kinds.
    let _: R = 1;
}
