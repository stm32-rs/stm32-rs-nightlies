///Register `STOPCR` reader
pub type R = crate::R<STOPCRrs>;
///Register `STOPCR` writer
pub type W = crate::W<STOPCRrs>;
///Field `LSISTOPEN` reader - LSI oscillator enable in Stop mode.
pub type LSISTOPEN_R = crate::BitReader;
///Field `LSISTOPEN` writer - LSI oscillator enable in Stop mode.
pub type LSISTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESTOPEN` reader - LSE oscillator enable in Stop mode.
pub type LSESTOPEN_R = crate::BitReader;
///Field `LSESTOPEN` writer - LSE oscillator enable in Stop mode.
pub type LSESTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISTOPEN` reader - MSI oscillator enable in Stop mode.
pub type MSISTOPEN_R = crate::BitReader;
///Field `MSISTOPEN` writer - MSI oscillator enable in Stop mode.
pub type MSISTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSISTOPEN` reader - HSI oscillator enable in Stop mode.
pub type HSISTOPEN_R = crate::BitReader;
///Field `HSISTOPEN` writer - HSI oscillator enable in Stop mode.
pub type HSISTOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn lsistopen(&self) -> LSISTOPEN_R {
        LSISTOPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator enable in Stop mode.
    #[inline(always)]
    pub fn lsestopen(&self) -> LSESTOPEN_R {
        LSESTOPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn msistopen(&self) -> MSISTOPEN_R {
        MSISTOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn hsistopen(&self) -> HSISTOPEN_R {
        HSISTOPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOPCR")
            .field("lsistopen", &self.lsistopen())
            .field("lsestopen", &self.lsestopen())
            .field("msistopen", &self.msistopen())
            .field("hsistopen", &self.hsistopen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn lsistopen(&mut self) -> LSISTOPEN_W<'_, STOPCRrs> {
        LSISTOPEN_W::new(self, 0)
    }
    ///Bit 1 - LSE oscillator enable in Stop mode.
    #[inline(always)]
    pub fn lsestopen(&mut self) -> LSESTOPEN_W<'_, STOPCRrs> {
        LSESTOPEN_W::new(self, 1)
    }
    ///Bit 2 - MSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn msistopen(&mut self) -> MSISTOPEN_W<'_, STOPCRrs> {
        MSISTOPEN_W::new(self, 2)
    }
    ///Bit 3 - HSI oscillator enable in Stop mode.
    #[inline(always)]
    pub fn hsistopen(&mut self) -> HSISTOPEN_W<'_, STOPCRrs> {
        HSISTOPEN_W::new(self, 3)
    }
}
/**RCC Stop mode control register

You can [`read`](crate::Reg::read) this register and get [`stopcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:STOPCR)*/
pub struct STOPCRrs;
impl crate::RegisterSpec for STOPCRrs {
    type Ux = u32;
}
///`read()` method returns [`stopcr::R`](R) reader structure
impl crate::Readable for STOPCRrs {}
///`write(|w| ..)` method takes [`stopcr::W`](W) writer structure
impl crate::Writable for STOPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STOPCR to value 0x08
impl crate::Resettable for STOPCRrs {
    const RESET_VALUE: u32 = 0x08;
}
