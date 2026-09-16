///Register `TSCSMPL_CR` reader
pub type R = crate::R<TSCSMPL_CRrs>;
///Register `TSCSMPL_CR` writer
pub type W = crate::W<TSCSMPL_CRrs>;
///Field `SMPL_CTR_DISABLE` reader - Sample counter disable bit
pub type SMPL_CTR_DISABLE_R = crate::BitReader;
///Field `SMPL_CTR_DISABLE` writer - Sample counter disable bit
pub type SMPL_CTR_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPL_CTR_HOLD` reader - Sample counter hold bit
pub type SMPL_CTR_HOLD_R = crate::BitReader;
///Field `SMPL_CTR_HOLD` writer - Sample counter hold bit
pub type SMPL_CTR_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPL_DISCARD` reader - Sample discard bit
pub type SMPL_DISCARD_R = crate::BitReader;
///Field `SMPL_DISCARD` writer - Sample discard bit
pub type SMPL_DISCARD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sample counter disable bit
    #[inline(always)]
    pub fn smpl_ctr_disable(&self) -> SMPL_CTR_DISABLE_R {
        SMPL_CTR_DISABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sample counter hold bit
    #[inline(always)]
    pub fn smpl_ctr_hold(&self) -> SMPL_CTR_HOLD_R {
        SMPL_CTR_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Sample discard bit
    #[inline(always)]
    pub fn smpl_discard(&self) -> SMPL_DISCARD_R {
        SMPL_DISCARD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSMPL_CR")
            .field("smpl_ctr_disable", &self.smpl_ctr_disable())
            .field("smpl_ctr_hold", &self.smpl_ctr_hold())
            .field("smpl_discard", &self.smpl_discard())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sample counter disable bit
    #[inline(always)]
    pub fn smpl_ctr_disable(&mut self) -> SMPL_CTR_DISABLE_W<'_, TSCSMPL_CRrs> {
        SMPL_CTR_DISABLE_W::new(self, 0)
    }
    ///Bit 1 - Sample counter hold bit
    #[inline(always)]
    pub fn smpl_ctr_hold(&mut self) -> SMPL_CTR_HOLD_W<'_, TSCSMPL_CRrs> {
        SMPL_CTR_HOLD_W::new(self, 1)
    }
    ///Bit 2 - Sample discard bit
    #[inline(always)]
    pub fn smpl_discard(&mut self) -> SMPL_DISCARD_W<'_, TSCSMPL_CRrs> {
        SMPL_DISCARD_W::new(self, 2)
    }
}
/**DTS TSC sample control register

You can [`read`](crate::Reg::read) this register and get [`tscsmpl_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsmpl_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DTS:TSCSMPL_CR)*/
pub struct TSCSMPL_CRrs;
impl crate::RegisterSpec for TSCSMPL_CRrs {
    type Ux = u32;
}
///`read()` method returns [`tscsmpl_cr::R`](R) reader structure
impl crate::Readable for TSCSMPL_CRrs {}
///`write(|w| ..)` method takes [`tscsmpl_cr::W`](W) writer structure
impl crate::Writable for TSCSMPL_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSMPL_CR to value 0
impl crate::Resettable for TSCSMPL_CRrs {}
