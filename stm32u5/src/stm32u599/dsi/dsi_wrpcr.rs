#[doc = "Register `DSI_WRPCR` reader"]
pub type R = crate::R<DSI_WRPCRrs>;
#[doc = "Register `DSI_WRPCR` writer"]
pub type W = crate::W<DSI_WRPCRrs>;
#[doc = "Field `PLLEN` reader - PLL enable This bit enables the D-PHY PLL."]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLL enable This bit enables the D-PHY PLL."]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDIV` reader - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2"]
pub type NDIV_R = crate::FieldReader<u16>;
#[doc = "Field `NDIV` writer - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2"]
pub type NDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `IDF` reader - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511"]
pub type IDF_R = crate::FieldReader<u16>;
#[doc = "Field `IDF` writer - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511"]
pub type IDF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ODF` reader - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511"]
pub type ODF_R = crate::FieldReader<u16>;
#[doc = "Field `ODF` writer - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511"]
pub type ODF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - PLL enable This bit enables the D-PHY PLL."]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:10 - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:19 - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511"]
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PLL enable This bit enables the D-PHY PLL."]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<DSI_WRPCRrs> {
        PLLEN_W::new(self, 0)
    }
    #[doc = "Bits 2:10 - PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<DSI_WRPCRrs> {
        NDIV_W::new(self, 2)
    }
    #[doc = "Bits 11:19 - PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511"]
    #[inline(always)]
    #[must_use]
    pub fn idf(&mut self) -> IDF_W<DSI_WRPCRrs> {
        IDF_W::new(self, 11)
    }
    #[doc = "Bits 20:28 - PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<DSI_WRPCRrs> {
        ODF_W::new(self, 20)
    }
}
#[doc = "DSI Wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wrpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wrpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WRPCRrs;
impl crate::RegisterSpec for DSI_WRPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wrpcr::R`](R) reader structure"]
impl crate::Readable for DSI_WRPCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_wrpcr::W`](W) writer structure"]
impl crate::Writable for DSI_WRPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_WRPCR to value 0"]
impl crate::Resettable for DSI_WRPCRrs {
    const RESET_VALUE: u32 = 0;
}
