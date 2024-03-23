#[doc = "Register `DSI_CCR` reader"]
pub type R = crate::R<DSI_CCRrs>;
#[doc = "Register `DSI_CCR` writer"]
pub type W = crate::W<DSI_CCRrs>;
#[doc = "Field `TXECKDIV` reader - TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
pub type TXECKDIV_R = crate::FieldReader;
#[doc = "Field `TXECKDIV` writer - TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
pub type TXECKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOCKDIV` reader - Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
pub type TOCKDIV_R = crate::FieldReader;
#[doc = "Field `TOCKDIV` writer - Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
pub type TOCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
    #[inline(always)]
    #[must_use]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W<DSI_CCRrs> {
        TXECKDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
    #[inline(always)]
    #[must_use]
    pub fn tockdiv(&mut self) -> TOCKDIV_W<DSI_CCRrs> {
        TOCKDIV_W::new(self, 8)
    }
}
#[doc = "DSI Host clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CCRrs;
impl crate::RegisterSpec for DSI_CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ccr::R`](R) reader structure"]
impl crate::Readable for DSI_CCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_ccr::W`](W) writer structure"]
impl crate::Writable for DSI_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_CCR to value 0"]
impl crate::Resettable for DSI_CCRrs {
    const RESET_VALUE: u32 = 0;
}
