///Register `WPCR5` reader
pub type R = crate::R<WPCR5rs>;
///Register `WPCR5` writer
pub type W = crate::W<WPCR5rs>;
///Field `THSZERO` reader - tCLK-POST
pub type THSZERO_R = crate::FieldReader;
///Field `THSZERO` writer - tCLK-POST
pub type THSZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - tCLK-POST
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR5")
            .field("thszero", &self.thszero())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - tCLK-POST
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W<WPCR5rs> {
        THSZERO_W::new(self, 0)
    }
}
/**DSI Wrapper PHY Configuration Register 5

You can [`read`](crate::Reg::read) this register and get [`wpcr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:WPCR5)*/
pub struct WPCR5rs;
impl crate::RegisterSpec for WPCR5rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr5::R`](R) reader structure
impl crate::Readable for WPCR5rs {}
///`write(|w| ..)` method takes [`wpcr5::W`](W) writer structure
impl crate::Writable for WPCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WPCR5 to value 0
impl crate::Resettable for WPCR5rs {
    const RESET_VALUE: u32 = 0;
}
