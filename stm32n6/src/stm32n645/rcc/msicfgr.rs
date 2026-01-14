///Register `MSICFGR` reader
pub type R = crate::R<MSICFGRrs>;
///Register `MSICFGR` writer
pub type W = crate::W<MSICFGRrs>;
///Field `MSIFREQSEL` reader - MSI oscillator frequency select
pub type MSIFREQSEL_R = crate::BitReader;
///Field `MSIFREQSEL` writer - MSI oscillator frequency select
pub type MSIFREQSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSITRIM` reader - MSI clock trimming
pub type MSITRIM_R = crate::FieldReader;
///Field `MSITRIM` writer - MSI clock trimming
pub type MSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `MSICAL` reader - MSI clock calibration
pub type MSICAL_R = crate::FieldReader;
impl R {
    ///Bit 9 - MSI oscillator frequency select
    #[inline(always)]
    pub fn msifreqsel(&self) -> MSIFREQSEL_R {
        MSIFREQSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:20 - MSI clock trimming
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 23:30 - MSI clock calibration
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 23) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSICFGR")
            .field("msifreqsel", &self.msifreqsel())
            .field("msitrim", &self.msitrim())
            .field("msical", &self.msical())
            .finish()
    }
}
impl W {
    ///Bit 9 - MSI oscillator frequency select
    #[inline(always)]
    pub fn msifreqsel(&mut self) -> MSIFREQSEL_W<'_, MSICFGRrs> {
        MSIFREQSEL_W::new(self, 9)
    }
    ///Bits 16:20 - MSI clock trimming
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W<'_, MSICFGRrs> {
        MSITRIM_W::new(self, 16)
    }
}
/**RCC MSI configuration register

You can [`read`](crate::Reg::read) this register and get [`msicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:MSICFGR)*/
pub struct MSICFGRrs;
impl crate::RegisterSpec for MSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`msicfgr::R`](R) reader structure
impl crate::Readable for MSICFGRrs {}
///`write(|w| ..)` method takes [`msicfgr::W`](W) writer structure
impl crate::Writable for MSICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSICFGR to value 0
impl crate::Resettable for MSICFGRrs {}
