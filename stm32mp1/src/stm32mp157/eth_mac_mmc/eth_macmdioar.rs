#[doc = "Register `ETH_MACMDIOAR` reader"]
pub type R = crate::R<ETH_MACMDIOARrs>;
#[doc = "Register `ETH_MACMDIOAR` writer"]
pub type W = crate::W<ETH_MACMDIOARrs>;
#[doc = "Field `GB` reader - GB"]
pub type GB_R = crate::BitReader;
#[doc = "Field `GB` writer - GB"]
pub type GB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C45E` reader - C45E"]
pub type C45E_R = crate::BitReader;
#[doc = "Field `C45E` writer - C45E"]
pub type C45E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOC` reader - GOC"]
pub type GOC_R = crate::FieldReader;
#[doc = "Field `GOC` writer - GOC"]
pub type GOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKAP` reader - SKAP"]
pub type SKAP_R = crate::BitReader;
#[doc = "Field `SKAP` writer - SKAP"]
pub type SKAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CR"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - CR"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NTC` reader - NTC"]
pub type NTC_R = crate::FieldReader;
#[doc = "Field `NTC` writer - NTC"]
pub type NTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDA` reader - RDA"]
pub type RDA_R = crate::FieldReader;
#[doc = "Field `RDA` writer - RDA"]
pub type RDA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PA"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PA"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BTB` reader - BTB"]
pub type BTB_R = crate::BitReader;
#[doc = "Field `BTB` writer - BTB"]
pub type BTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - PSE"]
pub type PSE_R = crate::BitReader;
#[doc = "Field `PSE` writer - PSE"]
pub type PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GB_W<ETH_MACMDIOARrs> {
        GB_W::new(self, 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<ETH_MACMDIOARrs> {
        C45E_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<ETH_MACMDIOARrs> {
        GOC_W::new(self, 2)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<ETH_MACMDIOARrs> {
        SKAP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<ETH_MACMDIOARrs> {
        CR_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<ETH_MACMDIOARrs> {
        NTC_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<ETH_MACMDIOARrs> {
        RDA_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<ETH_MACMDIOARrs> {
        PA_W::new(self, 21)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<ETH_MACMDIOARrs> {
        BTB_W::new(self, 26)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<ETH_MACMDIOARrs> {
        PSE_W::new(self, 27)
    }
}
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macmdioar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macmdioar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACMDIOARrs;
impl crate::RegisterSpec for ETH_MACMDIOARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macmdioar::R`](R) reader structure"]
impl crate::Readable for ETH_MACMDIOARrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macmdioar::W`](W) writer structure"]
impl crate::Writable for ETH_MACMDIOARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACMDIOAR to value 0"]
impl crate::Resettable for ETH_MACMDIOARrs {
    const RESET_VALUE: u32 = 0;
}
