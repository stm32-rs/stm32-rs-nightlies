///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
///Field `HSICAL` reader - HSI clock calibration
pub type HSICAL_R = crate::FieldReader<u16>;
///Field `HSITRIM` reader - HSI clock trimming
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - HSI clock trimming
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `CSICAL` reader - CSI clock calibration
pub type CSICAL_R = crate::FieldReader;
///Field `CSITRIM` reader - CSI clock trimming
pub type CSITRIM_R = crate::FieldReader;
///Field `CSITRIM` writer - CSI clock trimming
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:11 - HSI clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:17 - HSI clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 18:25 - CSI clock calibration
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    ///Bits 26:30 - CSI clock trimming
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .field("csical", &self.csical())
            .field("csitrim", &self.csitrim())
            .finish()
    }
}
impl W {
    ///Bits 12:17 - HSI clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<'_, ICSCRrs> {
        HSITRIM_W::new(self, 12)
    }
    ///Bits 26:30 - CSI clock trimming
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W<'_, ICSCRrs> {
        CSITRIM_W::new(self, 26)
    }
}
/**RCC Internal Clock Source Calibration Register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#RCC:ICSCR)*/
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
///`reset()` method sets ICSCR to value 0x4000_0000
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
