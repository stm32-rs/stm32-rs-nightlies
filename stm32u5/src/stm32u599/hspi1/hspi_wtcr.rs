#[doc = "Register `HSPI_WTCR` reader"]
pub type R = crate::R<HSPI_WTCRrs>;
#[doc = "Register `HSPI_WTCR` writer"]
pub type W = crate::W<HSPI_WTCRrs>;
#[doc = "Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
pub type DCYC_R = crate::FieldReader;
#[doc = "Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least 5 dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<HSPI_WTCRrs> {
        DCYC_W::new(self, 0)
    }
}
#[doc = "HSPI write timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_wtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_wtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_WTCRrs;
impl crate::RegisterSpec for HSPI_WTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_wtcr::R`](R) reader structure"]
impl crate::Readable for HSPI_WTCRrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_wtcr::W`](W) writer structure"]
impl crate::Writable for HSPI_WTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_WTCR to value 0"]
impl crate::Resettable for HSPI_WTCRrs {
    const RESET_VALUE: u32 = 0;
}
