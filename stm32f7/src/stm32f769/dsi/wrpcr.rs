///Register `WRPCR` reader
pub type R = crate::R<WRPCRrs>;
///Register `WRPCR` writer
pub type W = crate::W<WRPCRrs>;
///Field `PLLEN` reader - PLL Enable
pub type PLLEN_R = crate::BitReader;
///Field `PLLEN` writer - PLL Enable
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NDIV` reader - PLL Loop Division Factor
pub type NDIV_R = crate::FieldReader;
///Field `NDIV` writer - PLL Loop Division Factor
pub type NDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `IDF` reader - PLL Input Division Factor
pub type IDF_R = crate::FieldReader;
///Field `IDF` writer - PLL Input Division Factor
pub type IDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ODF` reader - PLL Output Division Factor
pub type ODF_R = crate::FieldReader;
///Field `ODF` writer - PLL Output Division Factor
pub type ODF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REGEN` reader - Regulator Enable
pub type REGEN_R = crate::BitReader;
///Field `REGEN` writer - Regulator Enable
pub type REGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLL Enable
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:8 - PLL Loop Division Factor
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    ///Bits 11:14 - PLL Input Division Factor
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 16:17 - PLL Output Division Factor
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Regulator Enable
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPCR")
            .field("regen", &self.regen())
            .field("odf", &self.odf())
            .field("idf", &self.idf())
            .field("ndiv", &self.ndiv())
            .field("pllen", &self.pllen())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL Enable
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<'_, WRPCRrs> {
        PLLEN_W::new(self, 0)
    }
    ///Bits 2:8 - PLL Loop Division Factor
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W<'_, WRPCRrs> {
        NDIV_W::new(self, 2)
    }
    ///Bits 11:14 - PLL Input Division Factor
    #[inline(always)]
    pub fn idf(&mut self) -> IDF_W<'_, WRPCRrs> {
        IDF_W::new(self, 11)
    }
    ///Bits 16:17 - PLL Output Division Factor
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W<'_, WRPCRrs> {
        ODF_W::new(self, 16)
    }
    ///Bit 24 - Regulator Enable
    #[inline(always)]
    pub fn regen(&mut self) -> REGEN_W<'_, WRPCRrs> {
        REGEN_W::new(self, 24)
    }
}
/**DSI Wrapper Regulator and PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`wrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WRPCR)*/
pub struct WRPCRrs;
impl crate::RegisterSpec for WRPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpcr::R`](R) reader structure
impl crate::Readable for WRPCRrs {}
///`write(|w| ..)` method takes [`wrpcr::W`](W) writer structure
impl crate::Writable for WRPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPCR to value 0
impl crate::Resettable for WRPCRrs {}
