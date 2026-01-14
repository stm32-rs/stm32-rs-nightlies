///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
///Field `MSICAL` reader - MSI clock calibration
pub type MSICAL_R = crate::FieldReader;
///Field `MSITRIM` reader - MSI clock trimming
pub type MSITRIM_R = crate::FieldReader;
///Field `MSITRIM` writer - MSI clock trimming
pub type MSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HSICAL` reader - HSI clock calibration
pub type HSICAL_R = crate::FieldReader;
///Field `HSITRIM` reader - HSI clock trimming
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - HSI clock trimming
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:7 - MSI clock calibration
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MSI clock trimming
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - HSI clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:28 - HSI clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("hsitrim", &self.hsitrim())
            .field("hsical", &self.hsical())
            .field("msitrim", &self.msitrim())
            .field("msical", &self.msical())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - MSI clock trimming
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W<'_, ICSCRrs> {
        MSITRIM_W::new(self, 8)
    }
    ///Bits 24:28 - HSI clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<'_, ICSCRrs> {
        HSITRIM_W::new(self, 24)
    }
}
/**Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:ICSCR)*/
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
///`read()` method returns [`icscr::R`](R) reader structure
impl crate::Readable for ICSCRrs {}
///`write(|w| ..)` method takes [`icscr::W`](W) writer structure
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR to value 0
impl crate::Resettable for ICSCRrs {}
