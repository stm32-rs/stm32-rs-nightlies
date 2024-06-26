///Register `GTZC1_MPCBB1_CR` reader
pub type R = crate::R<GTZC1_MPCBB1_CRrs>;
///Register `GTZC1_MPCBB1_CR` writer
pub type W = crate::W<GTZC1_MPCBB1_CRrs>;
///Field `GLOCK` reader - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
pub type GLOCK_R = crate::BitReader;
///Field `GLOCK` writer - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
pub type GLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INVSECSTATE` reader - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
pub type INVSECSTATE_R = crate::BitReader;
///Field `INVSECSTATE` writer - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
pub type INVSECSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRWILADIS` reader - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
pub type SRWILADIS_R = crate::BitReader;
///Field `SRWILADIS` writer - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
pub type SRWILADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTZC1_MPCBB1_CR")
            .field("glock", &self.glock())
            .field("invsecstate", &self.invsecstate())
            .field("srwiladis", &self.srwiladis())
            .finish()
    }
}
impl W {
    ///Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset.
    #[inline(always)]
    #[must_use]
    pub fn glock(&mut self) -> GLOCK_W<GTZC1_MPCBB1_CRrs> {
        GLOCK_W::new(self, 0)
    }
    ///Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
    #[inline(always)]
    #[must_use]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<GTZC1_MPCBB1_CRrs> {
        INVSECSTATE_W::new(self, 30)
    }
    ///Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal).
    #[inline(always)]
    #[must_use]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<GTZC1_MPCBB1_CRrs> {
        SRWILADIS_W::new(self, 31)
    }
}
/**GTZC1 SRAM1 MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`gtzc1_mpcbb1_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#GTZC1_MPCBB1:GTZC1_MPCBB1_CR)*/
pub struct GTZC1_MPCBB1_CRrs;
impl crate::RegisterSpec for GTZC1_MPCBB1_CRrs {
    type Ux = u32;
}
///`read()` method returns [`gtzc1_mpcbb1_cr::R`](R) reader structure
impl crate::Readable for GTZC1_MPCBB1_CRrs {}
///`write(|w| ..)` method takes [`gtzc1_mpcbb1_cr::W`](W) writer structure
impl crate::Writable for GTZC1_MPCBB1_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GTZC1_MPCBB1_CR to value 0
impl crate::Resettable for GTZC1_MPCBB1_CRrs {
    const RESET_VALUE: u32 = 0;
}
