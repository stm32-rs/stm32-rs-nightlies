#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `TI1_RMP` reader - Timer 10 input 1 remap"]
pub type TI1_RMP_R = crate::FieldReader;
#[doc = "Field `TI1_RMP` writer - Timer 10 input 1 remap"]
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETR_RMP` reader - Timer10 ETR remap"]
pub type ETR_RMP_R = crate::BitReader;
#[doc = "Field `ETR_RMP` writer - Timer10 ETR remap"]
pub type ETR_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI1_RMP_RI` reader - Timer10 Input 1 remap for Routing Interface (RI)"]
pub type TI1_RMP_RI_R = crate::BitReader;
#[doc = "Field `TI1_RMP_RI` writer - Timer10 Input 1 remap for Routing Interface (RI)"]
pub type TI1_RMP_RI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer 10 input 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Timer10 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer10 Input 1 remap for Routing Interface (RI)"]
    #[inline(always)]
    pub fn ti1_rmp_ri(&self) -> TI1_RMP_RI_R {
        TI1_RMP_RI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer 10 input 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<ORrs> {
        TI1_RMP_W::new(self, 0)
    }
    #[doc = "Bit 2 - Timer10 ETR remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<ORrs> {
        ETR_RMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer10 Input 1 remap for Routing Interface (RI)"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp_ri(&mut self) -> TI1_RMP_RI_W<ORrs> {
        TI1_RMP_RI_W::new(self, 3)
    }
}
#[doc = "option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
