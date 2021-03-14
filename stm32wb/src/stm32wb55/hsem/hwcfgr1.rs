#[doc = "Reader of register HWCFGR1"]
pub type R = crate::R<u32, super::HWCFGR1>;
#[doc = "Reader of field `NBINT`"]
pub type NBINT_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBSEM`"]
pub type NBSEM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:11 - Hardware Configuration number of interrupts supported number of master IDs"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - Hardware Configuration number of semaphores"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
}
