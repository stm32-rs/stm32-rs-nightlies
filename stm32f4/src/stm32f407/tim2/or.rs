///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**Timer Input 4 remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITR1_RMP {
    ///0: TIM8 trigger output is connected to TIM2_ITR1 input
    Tim8Trgout = 0,
    ///1: Ethernet PTP clock is connected to TIM2_ITR1 input
    Ptp = 1,
    ///2: OTG FS SOF is connected to the TIM2_ITR1 input
    OtgFsSof = 2,
    ///3: OTG HS SOF is connected to the TIM2_ITR1 input
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
impl crate::IsEnum for ITR1_RMP {}
///Field `ITR1_RMP` reader - Timer Input 4 remap
pub type ITR1_RMP_R = crate::FieldReader<ITR1_RMP>;
impl ITR1_RMP_R {
    ///Get enumerated values variant
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
    ///TIM8 trigger output is connected to TIM2_ITR1 input
    #[inline(always)]
    pub fn is_tim8_trgout(&self) -> bool {
        *self == ITR1_RMP::Tim8Trgout
    }
    ///Ethernet PTP clock is connected to TIM2_ITR1 input
    #[inline(always)]
    pub fn is_ptp(&self) -> bool {
        *self == ITR1_RMP::Ptp
    }
    ///OTG FS SOF is connected to the TIM2_ITR1 input
    #[inline(always)]
    pub fn is_otg_fs_sof(&self) -> bool {
        *self == ITR1_RMP::OtgFsSof
    }
    ///OTG HS SOF is connected to the TIM2_ITR1 input
    #[inline(always)]
    pub fn is_otg_hs_sof(&self) -> bool {
        *self == ITR1_RMP::OtgHsSof
    }
}
///Field `ITR1_RMP` writer - Timer Input 4 remap
pub type ITR1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITR1_RMP, crate::Safe>;
impl<'a, REG> ITR1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM8 trigger output is connected to TIM2_ITR1 input
    #[inline(always)]
    pub fn tim8_trgout(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::Tim8Trgout)
    }
    ///Ethernet PTP clock is connected to TIM2_ITR1 input
    #[inline(always)]
    pub fn ptp(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::Ptp)
    }
    ///OTG FS SOF is connected to the TIM2_ITR1 input
    #[inline(always)]
    pub fn otg_fs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::OtgFsSof)
    }
    ///OTG HS SOF is connected to the TIM2_ITR1 input
    #[inline(always)]
    pub fn otg_hs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(ITR1_RMP::OtgHsSof)
    }
}
impl R {
    ///Bits 10:11 - Timer Input 4 remap
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("itr1_rmp", &self.itr1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 10:11 - Timer Input 4 remap
    #[inline(always)]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<'_, ORrs> {
        ITR1_RMP_W::new(self, 10)
    }
}
/**TIM5 option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#TIM2:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u16;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
