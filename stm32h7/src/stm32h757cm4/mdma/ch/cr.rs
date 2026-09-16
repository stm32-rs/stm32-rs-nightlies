///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - channel enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - channel enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIE` reader - Channel Transfer Complete interrupt enable This bit is set and cleared by software.
pub type CTCIE_R = crate::BitReader;
///Field `CTCIE` writer - Channel Transfer Complete interrupt enable This bit is set and cleared by software.
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRTIE` reader - Block Repeat transfer interrupt enable This bit is set and cleared by software.
pub type BRTIE_R = crate::BitReader;
///Field `BRTIE` writer - Block Repeat transfer interrupt enable This bit is set and cleared by software.
pub type BRTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTIE` reader - Block Transfer interrupt enable This bit is set and cleared by software.
pub type BTIE_R = crate::BitReader;
///Field `BTIE` writer - Block Transfer interrupt enable This bit is set and cleared by software.
pub type BTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - buffer Transfer Complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - buffer Transfer Complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PL` reader - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.
pub type PL_R = crate::FieldReader;
///Field `PL` writer - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BEX` reader - byte Endianness exchange
pub type BEX_R = crate::BitReader;
///Field `BEX` writer - byte Endianness exchange
pub type BEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HEX` reader - Half word Endianes exchange
pub type HEX_R = crate::BitReader;
///Field `HEX` writer - Half word Endianes exchange
pub type HEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WEX` reader - Word Endianness exchange
pub type WEX_R = crate::BitReader;
///Field `WEX` writer - Word Endianness exchange
pub type WEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWRQ` writer - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access).
pub type SWRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 12 - byte Endianness exchange
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Half word Endianes exchange
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Word Endianness exchange
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
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
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 1)
    }
    ///Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W<'_, CRrs> {
        CTCIE_W::new(self, 2)
    }
    ///Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn brtie(&mut self) -> BRTIE_W<'_, CRrs> {
        BRTIE_W::new(self, 3)
    }
    ///Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn btie(&mut self) -> BTIE_W<'_, CRrs> {
        BTIE_W::new(self, 4)
    }
    ///Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 5)
    }
    ///Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, CRrs> {
        PL_W::new(self, 6)
    }
    ///Bit 12 - byte Endianness exchange
    #[inline(always)]
    pub fn bex(&mut self) -> BEX_W<'_, CRrs> {
        BEX_W::new(self, 12)
    }
    ///Bit 13 - Half word Endianes exchange
    #[inline(always)]
    pub fn hex(&mut self) -> HEX_W<'_, CRrs> {
        HEX_W::new(self, 13)
    }
    ///Bit 14 - Word Endianness exchange
    #[inline(always)]
    pub fn wex(&mut self) -> WEX_W<'_, CRrs> {
        WEX_W::new(self, 14)
    }
    ///Bit 16 - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access).
    #[inline(always)]
    pub fn swrq(&mut self) -> SWRQ_W<'_, CRrs> {
        SWRQ_W::new(self, 16)
    }
}
/**This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
