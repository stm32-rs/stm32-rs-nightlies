///Register `ICSCR` reader
pub type R = crate::R<ICSCRrs>;
///Register `ICSCR` writer
pub type W = crate::W<ICSCRrs>;
///Field `HSICAL0` reader - Internal High Speed clock Calibration
pub type HSICAL0_R = crate::FieldReader;
///Field `HSITRIM` reader - Internal High Speed clock trimming
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - Internal High Speed clock trimming
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 16:23 - Internal High Speed clock Calibration
    #[inline(always)]
    pub fn hsical0(&self) -> HSICAL0_R {
        HSICAL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - Internal High Speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR")
            .field("hsical0", &self.hsical0())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    ///Bits 24:30 - Internal High Speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<ICSCRrs> {
        HSITRIM_W::new(self, 24)
    }
}
/**Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473xx.html#RCC:ICSCR)*/
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
///`read()` method returns [`icscr::R`](R) reader structure
impl crate::Readable for ICSCRrs {}
///`write(|w| ..)` method takes [`icscr::W`](W) writer structure
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICSCR to value 0x4000_0000
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
