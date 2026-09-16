///Register `C7CR` reader
pub type R = crate::R<C7CRrs>;
///Register `C7CR` writer
pub type W = crate::W<C7CRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - TEIE
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - TEIE
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIE` reader - CTCIE
pub type CTCIE_R = crate::BitReader;
///Field `CTCIE` writer - CTCIE
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRTIE` reader - BRTIE
pub type BRTIE_R = crate::BitReader;
///Field `BRTIE` writer - BRTIE
pub type BRTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIE` reader - BTIE
pub type BTIE_R = crate::BitReader;
///Field `BTIE` writer - BTIE
pub type BTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PL` reader - PL
pub type PL_R = crate::FieldReader;
///Field `PL` writer - PL
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BEX` reader - BEX
pub type BEX_R = crate::BitReader;
///Field `BEX` writer - BEX
pub type BEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEX` reader - HEX
pub type HEX_R = crate::BitReader;
///Field `HEX` writer - HEX
pub type HEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WEX` reader - WEX
pub type WEX_R = crate::BitReader;
///Field `WEX` writer - WEX
pub type WEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWRQ` writer - SWRQ
pub type SWRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TEIE
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTCIE
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRTIE
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BTIE
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - PL
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 12 - BEX
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HEX
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WEX
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C7CR")
            .field("en", &self.en())
            .field("teie", &self.teie())
            .field("ctcie", &self.ctcie())
            .field("brtie", &self.brtie())
            .field("btie", &self.btie())
            .field("tcie", &self.tcie())
            .field("pl", &self.pl())
            .field("bex", &self.bex())
            .field("hex", &self.hex())
            .field("wex", &self.wex())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, C7CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - TEIE
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, C7CRrs> {
        TEIE_W::new(self, 1)
    }
    ///Bit 2 - CTCIE
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W<'_, C7CRrs> {
        CTCIE_W::new(self, 2)
    }
    ///Bit 3 - BRTIE
    #[inline(always)]
    pub fn brtie(&mut self) -> BRTIE_W<'_, C7CRrs> {
        BRTIE_W::new(self, 3)
    }
    ///Bit 4 - BTIE
    #[inline(always)]
    pub fn btie(&mut self) -> BTIE_W<'_, C7CRrs> {
        BTIE_W::new(self, 4)
    }
    ///Bit 5 - TCIE
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, C7CRrs> {
        TCIE_W::new(self, 5)
    }
    ///Bits 6:7 - PL
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, C7CRrs> {
        PL_W::new(self, 6)
    }
    ///Bit 12 - BEX
    #[inline(always)]
    pub fn bex(&mut self) -> BEX_W<'_, C7CRrs> {
        BEX_W::new(self, 12)
    }
    ///Bit 13 - HEX
    #[inline(always)]
    pub fn hex(&mut self) -> HEX_W<'_, C7CRrs> {
        HEX_W::new(self, 13)
    }
    ///Bit 14 - WEX
    #[inline(always)]
    pub fn wex(&mut self) -> WEX_W<'_, C7CRrs> {
        WEX_W::new(self, 14)
    }
    ///Bit 16 - SWRQ
    #[inline(always)]
    pub fn swrq(&mut self) -> SWRQ_W<'_, C7CRrs> {
        SWRQ_W::new(self, 16)
    }
}
/**This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C7CR)*/
pub struct C7CRrs;
impl crate::RegisterSpec for C7CRrs {
    type Ux = u32;
}
///`read()` method returns [`c7cr::R`](R) reader structure
impl crate::Readable for C7CRrs {}
///`write(|w| ..)` method takes [`c7cr::W`](W) writer structure
impl crate::Writable for C7CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C7CR to value 0
impl crate::Resettable for C7CRrs {}
