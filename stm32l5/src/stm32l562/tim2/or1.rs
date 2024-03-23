#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1rs>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1rs>;
#[doc = "Field `ITR1_RMP` reader - Internal trigger 1 remap"]
pub type ITR1_RMP_R = crate::BitReader;
#[doc = "Field `ITR1_RMP` writer - Internal trigger 1 remap"]
pub type ITR1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETR1_RMP` reader - External trigger remap"]
pub type ETR1_RMP_R = crate::BitReader;
#[doc = "Field `ETR1_RMP` writer - External trigger remap"]
pub type ETR1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI4_RMP` reader - Input Capture 4 remap"]
pub type TI4_RMP_R = crate::FieldReader;
#[doc = "Field `TI4_RMP` writer - Input Capture 4 remap"]
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Internal trigger 1 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr1_rmp(&self) -> ETR1_RMP_R {
        ETR1_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Input Capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal trigger 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<OR1rs> {
        ITR1_RMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr1_rmp(&mut self) -> ETR1_RMP_W<OR1rs> {
        ETR1_RMP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Input Capture 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<OR1rs> {
        TI4_RMP_W::new(self, 2)
    }
}
#[doc = "TIM2 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1rs {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1rs {
    const RESET_VALUE: u32 = 0;
}
