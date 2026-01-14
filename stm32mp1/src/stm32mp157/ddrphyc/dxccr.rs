///Register `DXCCR` reader
pub type R = crate::R<DXCCRrs>;
///Register `DXCCR` writer
pub type W = crate::W<DXCCRrs>;
///Field `DXODT` reader - DXODT
pub type DXODT_R = crate::BitReader;
///Field `DXODT` writer - DXODT
pub type DXODT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXIOM` reader - DXIOM
pub type DXIOM_R = crate::BitReader;
///Field `DXIOM` writer - DXIOM
pub type DXIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXPDD` reader - DXPDD
pub type DXPDD_R = crate::BitReader;
///Field `DXPDD` writer - DXPDD
pub type DXPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXPDR` reader - DXPDR
pub type DXPDR_R = crate::BitReader;
///Field `DXPDR` writer - DXPDR
pub type DXPDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSRES` reader - DQSRES
pub type DQSRES_R = crate::FieldReader;
///Field `DQSRES` writer - DQSRES
pub type DQSRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQSNRES` reader - DQSNRES
pub type DQSNRES_R = crate::FieldReader;
///Field `DQSNRES` writer - DQSNRES
pub type DQSNRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DQSNRST` reader - DQSNRST
pub type DQSNRST_R = crate::BitReader;
///Field `DQSNRST` writer - DQSNRST
pub type DQSNRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RVSEL` reader - RVSEL
pub type RVSEL_R = crate::BitReader;
///Field `RVSEL` writer - RVSEL
pub type RVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWDT` reader - AWDT
pub type AWDT_R = crate::BitReader;
///Field `AWDT` writer - AWDT
pub type AWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DXODT
    #[inline(always)]
    pub fn dxodt(&self) -> DXODT_R {
        DXODT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DXIOM
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXPDD
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DXPDR
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DQSRES
    #[inline(always)]
    pub fn dqsres(&self) -> DQSRES_R {
        DQSRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DQSNRES
    #[inline(always)]
    pub fn dqsnres(&self) -> DQSNRES_R {
        DQSNRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 14 - DQSNRST
    #[inline(always)]
    pub fn dqsnrst(&self) -> DQSNRST_R {
        DQSNRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RVSEL
    #[inline(always)]
    pub fn rvsel(&self) -> RVSEL_R {
        RVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AWDT
    #[inline(always)]
    pub fn awdt(&self) -> AWDT_R {
        AWDT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DXCCR")
            .field("dxodt", &self.dxodt())
            .field("dxiom", &self.dxiom())
            .field("dxpdd", &self.dxpdd())
            .field("dxpdr", &self.dxpdr())
            .field("dqsres", &self.dqsres())
            .field("dqsnres", &self.dqsnres())
            .field("dqsnrst", &self.dqsnrst())
            .field("rvsel", &self.rvsel())
            .field("awdt", &self.awdt())
            .finish()
    }
}
impl W {
    ///Bit 0 - DXODT
    #[inline(always)]
    pub fn dxodt(&mut self) -> DXODT_W<'_, DXCCRrs> {
        DXODT_W::new(self, 0)
    }
    ///Bit 1 - DXIOM
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W<'_, DXCCRrs> {
        DXIOM_W::new(self, 1)
    }
    ///Bit 2 - DXPDD
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W<'_, DXCCRrs> {
        DXPDD_W::new(self, 2)
    }
    ///Bit 3 - DXPDR
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W<'_, DXCCRrs> {
        DXPDR_W::new(self, 3)
    }
    ///Bits 4:7 - DQSRES
    #[inline(always)]
    pub fn dqsres(&mut self) -> DQSRES_W<'_, DXCCRrs> {
        DQSRES_W::new(self, 4)
    }
    ///Bits 8:11 - DQSNRES
    #[inline(always)]
    pub fn dqsnres(&mut self) -> DQSNRES_W<'_, DXCCRrs> {
        DQSNRES_W::new(self, 8)
    }
    ///Bit 14 - DQSNRST
    #[inline(always)]
    pub fn dqsnrst(&mut self) -> DQSNRST_W<'_, DXCCRrs> {
        DQSNRST_W::new(self, 14)
    }
    ///Bit 15 - RVSEL
    #[inline(always)]
    pub fn rvsel(&mut self) -> RVSEL_W<'_, DXCCRrs> {
        RVSEL_W::new(self, 15)
    }
    ///Bit 16 - AWDT
    #[inline(always)]
    pub fn awdt(&mut self) -> AWDT_W<'_, DXCCRrs> {
        AWDT_W::new(self, 16)
    }
}
/**DDRPHYC DXCC register

You can [`read`](crate::Reg::read) this register and get [`dxccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dxccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DXCCR)*/
pub struct DXCCRrs;
impl crate::RegisterSpec for DXCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dxccr::R`](R) reader structure
impl crate::Readable for DXCCRrs {}
///`write(|w| ..)` method takes [`dxccr::W`](W) writer structure
impl crate::Writable for DXCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DXCCR to value 0x0800
impl crate::Resettable for DXCCRrs {
    const RESET_VALUE: u32 = 0x0800;
}
