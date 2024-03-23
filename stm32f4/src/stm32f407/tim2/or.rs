#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Timer Input 4 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITR1_RMP {
    #[doc = "0: TIM8 trigger output is connected to TIM2_ITR1 input"]
    Tim8Trgout = 0,
    #[doc = "1: Ethernet PTP clock is connected to TIM2_ITR1 input"]
    Ptp = 1,
    #[doc = "2: OTG FS SOF is connected to the TIM2_ITR1 input"]
    OtgFsSof = 2,
    #[doc = "3: OTG HS SOF is connected to the TIM2_ITR1 input"]
    OtgHsSof = 3,
}
impl From<ITR1_RMP> for u8 {
    #[inline(always)]
    fn from(variant: ITR1_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITR1_RMP {
    type Ux = u8;
}
#[doc = "Field `ITR1_RMP` reader - Timer Input 4 remap"]
pub type ITR1_RMP_R = crate::FieldReader<ITR1_RMP>;
impl ITR1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITR1_RMP {
        match self.bits {
            0 => ITR1_RMP::Tim8Trgout,
            1 => ITR1_RMP::Ptp,
            2 => ITR1_RMP::OtgFsSof,
            3 => ITR1_RMP::OtgHsSof,
            _ => unreachable!(),
        }
    }
    #[doc = "TIM8 trigger output is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn is_tim8_trgout(&self) -> bool {
        *self == ITR1_RMP::Tim8Trgout
    }
    #[doc = "Ethernet PTP clock is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn is_ptp(&self) -> bool {
        *self == ITR1_RMP::Ptp
    }
    #[doc = "OTG FS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn is_otg_fs_sof(&self) -> bool {
        *self == ITR1_RMP::OtgFsSof
    }
    #[doc = "OTG HS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn is_otg_hs_sof(&self) -> bool {
        *self == ITR1_RMP::OtgHsSof
    }
}
#[doc = "Field `ITR1_RMP` writer - Timer Input 4 remap"]
pub type ITR1_RMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ITR1_RMP>;
impl<'a, REG> ITR1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM8 trigger output is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn tim8_trgout(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::Tim8Trgout)
    }
    #[doc = "Ethernet PTP clock is connected to TIM2_ITR1 input"]
    #[inline(always)]
    pub fn ptp(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::Ptp)
    }
    #[doc = "OTG FS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn otg_fs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::OtgFsSof)
    }
    #[doc = "OTG HS SOF is connected to the TIM2_ITR1 input"]
    #[inline(always)]
    pub fn otg_hs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::OtgHsSof)
    }
}
impl R {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Timer Input 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<ORrs> {
        ITR1_RMP_W::new(self, 10)
    }
}
#[doc = "TIM5 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
