///Register `CM55CR` reader
pub type R = crate::R<CM55CRrs>;
///Register `CM55CR` writer
pub type W = crate::W<CM55CRrs>;
///Field `FPU_IT_EN` reader - Enable FPU exception
pub type FPU_IT_EN_R = crate::FieldReader;
///Field `FPU_IT_EN` writer - Enable FPU exception
pub type FPU_IT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `LOCKSVTAIRCR` reader - Prevent changes to:
pub type LOCKSVTAIRCR_R = crate::BitReader;
///Field `LOCKSVTAIRCR` writer - Prevent changes to:
pub type LOCKSVTAIRCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKNSVTOR` reader - Prevent changes to the non-secure vector table base address.
pub type LOCKNSVTOR_R = crate::BitReader;
///Field `LOCKNSVTOR` writer - Prevent changes to the non-secure vector table base address.
pub type LOCKNSVTOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSMPU` reader - Prevent changes to programmed secure MPU memory regions.
pub type LOCKSMPU_R = crate::BitReader;
///Field `LOCKSMPU` writer - Prevent changes to programmed secure MPU memory regions.
pub type LOCKSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKNSMPU` reader - Prevent changes to non-secure MPU memory regions already programmed.
pub type LOCKNSMPU_R = crate::BitReader;
///Field `LOCKNSMPU` writer - Prevent changes to non-secure MPU memory regions already programmed.
pub type LOCKNSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSAU` reader - Prevent changes to secure SAU memory regions already programmed.
pub type LOCKSAU_R = crate::BitReader;
///Field `LOCKSAU` writer - Prevent changes to secure SAU memory regions already programmed.
pub type LOCKSAU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKDCAIC` reader - Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR.
pub type LOCKDCAIC_R = crate::BitReader;
///Field `LOCKDCAIC` writer - Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR.
pub type LOCKDCAIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Enable FPU exception
    #[inline(always)]
    pub fn fpu_it_en(&self) -> FPU_IT_EN_R {
        FPU_IT_EN_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - Prevent changes to:
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Prevent changes to the non-secure vector table base address.
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Prevent changes to programmed secure MPU memory regions.
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Prevent changes to non-secure MPU memory regions already programmed.
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Prevent changes to secure SAU memory regions already programmed.
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR.
    #[inline(always)]
    pub fn lockdcaic(&self) -> LOCKDCAIC_R {
        LOCKDCAIC_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM55CR")
            .field("fpu_it_en", &self.fpu_it_en())
            .field("locksvtaircr", &self.locksvtaircr())
            .field("locknsvtor", &self.locknsvtor())
            .field("locksmpu", &self.locksmpu())
            .field("locknsmpu", &self.locknsmpu())
            .field("locksau", &self.locksau())
            .field("lockdcaic", &self.lockdcaic())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Enable FPU exception
    #[inline(always)]
    pub fn fpu_it_en(&mut self) -> FPU_IT_EN_W<'_, CM55CRrs> {
        FPU_IT_EN_W::new(self, 0)
    }
    ///Bit 16 - Prevent changes to:
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<'_, CM55CRrs> {
        LOCKSVTAIRCR_W::new(self, 16)
    }
    ///Bit 17 - Prevent changes to the non-secure vector table base address.
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<'_, CM55CRrs> {
        LOCKNSVTOR_W::new(self, 17)
    }
    ///Bit 18 - Prevent changes to programmed secure MPU memory regions.
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<'_, CM55CRrs> {
        LOCKSMPU_W::new(self, 18)
    }
    ///Bit 19 - Prevent changes to non-secure MPU memory regions already programmed.
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<'_, CM55CRrs> {
        LOCKNSMPU_W::new(self, 19)
    }
    ///Bit 20 - Prevent changes to secure SAU memory regions already programmed.
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W<'_, CM55CRrs> {
        LOCKSAU_W::new(self, 20)
    }
    ///Bit 21 - Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR.
    #[inline(always)]
    pub fn lockdcaic(&mut self) -> LOCKDCAIC_W<'_, CM55CRrs> {
        LOCKDCAIC_W::new(self, 21)
    }
}
/**SYSCFG Cortex-M55 control register

You can [`read`](crate::Reg::read) this register and get [`cm55cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CM55CR)*/
pub struct CM55CRrs;
impl crate::RegisterSpec for CM55CRrs {
    type Ux = u32;
}
///`read()` method returns [`cm55cr::R`](R) reader structure
impl crate::Readable for CM55CRrs {}
///`write(|w| ..)` method takes [`cm55cr::W`](W) writer structure
impl crate::Writable for CM55CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM55CR to value 0
impl crate::Resettable for CM55CRrs {}
