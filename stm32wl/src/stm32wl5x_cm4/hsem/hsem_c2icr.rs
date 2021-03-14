#[doc = "Reader of register HSEM_C2ICR"]
pub type R = crate::R<u32, super::HSEM_C2ICR>;
#[doc = "Reader of field `ISC0`"]
pub type ISC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC1`"]
pub type ISC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC2`"]
pub type ISC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC3`"]
pub type ISC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC4`"]
pub type ISC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC5`"]
pub type ISC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC6`"]
pub type ISC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC7`"]
pub type ISC7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC8`"]
pub type ISC8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC9`"]
pub type ISC9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC10`"]
pub type ISC10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC11`"]
pub type ISC11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC12`"]
pub type ISC12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC13`"]
pub type ISC13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC14`"]
pub type ISC14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISC15`"]
pub type ISC15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc8(&self) -> ISC8_R {
        ISC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc9(&self) -> ISC9_R {
        ISC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc10(&self) -> ISC10_R {
        ISC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc11(&self) -> ISC11_R {
        ISC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc12(&self) -> ISC12_R {
        ISC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc13(&self) -> ISC13_R {
        ISC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc14(&self) -> ISC14_R {
        ISC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc15(&self) -> ISC15_R {
        ISC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
