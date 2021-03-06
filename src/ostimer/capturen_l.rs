#[doc = "Reader of register CAPTUREn_L"]
pub type R = crate::R<u32, super::CAPTUREN_L>;
#[doc = "Reader of field `CAPTUREn_VALUE`"]
pub type CAPTUREN_VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - A read reflects the value of the lower 32 bits of the central EVTIMER at the time the last capture signal was generated by the CPU. A separate pair of CAPTURE registers are implemented for each CPU. Each CPU reads its own capture value at the same pair of addresses."]
    #[inline(always)]
    pub fn capturen_value(&self) -> CAPTUREN_VALUE_R {
        CAPTUREN_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
