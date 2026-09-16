///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `STOPWUCK` reader - System clock selection after a wake up from system Stop.
pub type STOPWUCK_R = crate::BitReader;
///Field `STOPWUCK` writer - System clock selection after a wake up from system Stop.
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPUSW` reader - CPU clock switch selection
pub type CPUSW_R = crate::FieldReader;
///Field `CPUSW` writer - CPU clock switch selection
pub type CPUSW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPUSWS` reader - CPU clock switch status
pub type CPUSWS_R = crate::FieldReader;
///Field `SYSSW` reader - System clock switch selection
pub type SYSSW_R = crate::FieldReader;
///Field `SYSSW` writer - System clock switch selection
pub type SYSSW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYSSWS` reader - System clock switch status
pub type SYSSWS_R = crate::FieldReader;
impl R {
    ///Bit 0 - System clock selection after a wake up from system Stop.
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:17 - CPU clock switch selection
    #[inline(always)]
    pub fn cpusw(&self) -> CPUSW_R {
        CPUSW_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - CPU clock switch status
    #[inline(always)]
    pub fn cpusws(&self) -> CPUSWS_R {
        CPUSWS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - System clock switch selection
    #[inline(always)]
    pub fn syssw(&self) -> SYSSW_R {
        SYSSW_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - System clock switch status
    #[inline(always)]
    pub fn syssws(&self) -> SYSSWS_R {
        SYSSWS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("stopwuck", &self.stopwuck())
            .field("cpusw", &self.cpusw())
            .field("cpusws", &self.cpusws())
            .field("syssw", &self.syssw())
            .field("syssws", &self.syssws())
            .finish()
    }
}
impl W {
    ///Bit 0 - System clock selection after a wake up from system Stop.
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<'_, CFGR1rs> {
        STOPWUCK_W::new(self, 0)
    }
    ///Bits 16:17 - CPU clock switch selection
    #[inline(always)]
    pub fn cpusw(&mut self) -> CPUSW_W<'_, CFGR1rs> {
        CPUSW_W::new(self, 16)
    }
    ///Bits 24:25 - System clock switch selection
    #[inline(always)]
    pub fn syssw(&mut self) -> SYSSW_W<'_, CFGR1rs> {
        SYSSW_W::new(self, 24)
    }
}
/**RCC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
