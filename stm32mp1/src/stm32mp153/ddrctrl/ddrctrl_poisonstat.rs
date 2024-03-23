#[doc = "Register `DDRCTRL_POISONSTAT` reader"]
pub type R = crate::R<DDRCTRL_POISONSTATrs>;
#[doc = "Field `WR_POISON_INTR_0` reader - WR_POISON_INTR_0"]
pub type WR_POISON_INTR_0_R = crate::BitReader;
#[doc = "Field `WR_POISON_INTR_1` reader - WR_POISON_INTR_1"]
pub type WR_POISON_INTR_1_R = crate::BitReader;
#[doc = "Field `RD_POISON_INTR_0` reader - RD_POISON_INTR_0"]
pub type RD_POISON_INTR_0_R = crate::BitReader;
#[doc = "Field `RD_POISON_INTR_1` reader - RD_POISON_INTR_1"]
pub type RD_POISON_INTR_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - WR_POISON_INTR_0"]
    #[inline(always)]
    pub fn wr_poison_intr_0(&self) -> WR_POISON_INTR_0_R {
        WR_POISON_INTR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WR_POISON_INTR_1"]
    #[inline(always)]
    pub fn wr_poison_intr_1(&self) -> WR_POISON_INTR_1_R {
        WR_POISON_INTR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_INTR_0"]
    #[inline(always)]
    pub fn rd_poison_intr_0(&self) -> RD_POISON_INTR_0_R {
        RD_POISON_INTR_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RD_POISON_INTR_1"]
    #[inline(always)]
    pub fn rd_poison_intr_1(&self) -> RD_POISON_INTR_1_R {
        RD_POISON_INTR_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DDRCTRL AXI Poison status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_poisonstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_POISONSTATrs;
impl crate::RegisterSpec for DDRCTRL_POISONSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_poisonstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_POISONSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_POISONSTAT to value 0"]
impl crate::Resettable for DDRCTRL_POISONSTATrs {
    const RESET_VALUE: u32 = 0;
}
