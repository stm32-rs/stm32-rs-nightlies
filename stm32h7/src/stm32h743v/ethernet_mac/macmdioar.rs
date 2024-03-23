#[doc = "Register `MACMDIOAR` reader"]
pub type R = crate::R<MACMDIOARrs>;
#[doc = "Register `MACMDIOAR` writer"]
pub type W = crate::W<MACMDIOARrs>;
#[doc = "Field `MB` reader - MII Busy"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII Busy"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C45E` reader - Clause 45 PHY Enable"]
pub type C45E_R = crate::BitReader;
#[doc = "Field `C45E` writer - Clause 45 PHY Enable"]
pub type C45E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOC` reader - MII Operation Command"]
pub type GOC_R = crate::FieldReader;
#[doc = "Field `GOC` writer - MII Operation Command"]
pub type GOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKAP` reader - Skip Address Packet"]
pub type SKAP_R = crate::BitReader;
#[doc = "Field `SKAP` writer - Skip Address Packet"]
pub type SKAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CSR Clock Range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - CSR Clock Range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NTC` reader - Number of Training Clocks"]
pub type NTC_R = crate::FieldReader;
#[doc = "Field `NTC` writer - Number of Training Clocks"]
pub type NTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDA` reader - Register/Device Address"]
pub type RDA_R = crate::FieldReader;
#[doc = "Field `RDA` writer - Register/Device Address"]
pub type RDA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BTB` reader - Back to Back transactions"]
pub type BTB_R = crate::BitReader;
#[doc = "Field `BTB` writer - Back to Back transactions"]
pub type BTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - Preamble Suppression Enable"]
pub type PSE_R = crate::BitReader;
#[doc = "Field `PSE` writer - Preamble Suppression Enable"]
pub type PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MACMDIOARrs> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<MACMDIOARrs> {
        C45E_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<MACMDIOARrs> {
        GOC_W::new(self, 2)
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<MACMDIOARrs> {
        SKAP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MACMDIOARrs> {
        CR_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<MACMDIOARrs> {
        NTC_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<MACMDIOARrs> {
        RDA_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MACMDIOARrs> {
        PA_W::new(self, 21)
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<MACMDIOARrs> {
        BTB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<MACMDIOARrs> {
        PSE_W::new(self, 27)
    }
}
#[doc = "MDIO address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdioar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdioar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMDIOARrs;
impl crate::RegisterSpec for MACMDIOARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdioar::R`](R) reader structure"]
impl crate::Readable for MACMDIOARrs {}
#[doc = "`write(|w| ..)` method takes [`macmdioar::W`](W) writer structure"]
impl crate::Writable for MACMDIOARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MACMDIOARrs {
    const RESET_VALUE: u32 = 0;
}
