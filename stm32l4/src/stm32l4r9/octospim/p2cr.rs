#[doc = "Register `P2CR` reader"]
pub type R = crate::R<P2CRrs>;
#[doc = "Register `P2CR` writer"]
pub type W = crate::W<P2CRrs>;
#[doc = "Field `CLKEN` reader - CLK/CLK Enable for Port"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - CLK/CLK Enable for Port"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSRC` reader - CLK/CLK Source for Port"]
pub type CLKSRC_R = crate::BitReader;
#[doc = "Field `CLKSRC` writer - CLK/CLK Source for Port"]
pub type CLKSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSEN` reader - DQS Enable for Port"]
pub type DQSEN_R = crate::BitReader;
#[doc = "Field `DQSEN` writer - DQS Enable for Port"]
pub type DQSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSSRC` reader - DQS Source for Port"]
pub type DQSSRC_R = crate::BitReader;
#[doc = "Field `DQSSRC` writer - DQS Source for Port"]
pub type DQSSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCSEN` reader - CS Enable for Port"]
pub type NCSEN_R = crate::BitReader;
#[doc = "Field `NCSEN` writer - CS Enable for Port"]
pub type NCSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCSSRC` reader - CS Source for Port"]
pub type NCSSRC_R = crate::BitReader;
#[doc = "Field `NCSSRC` writer - CS Source for Port"]
pub type NCSSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLEN` reader - Enable for Port"]
pub type IOLEN_R = crate::BitReader;
#[doc = "Field `IOLEN` writer - Enable for Port"]
pub type IOLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLSRC` reader - Source for Port"]
pub type IOLSRC_R = crate::FieldReader;
#[doc = "Field `IOLSRC` writer - Source for Port"]
pub type IOLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOHEN` reader - Enable for Port n"]
pub type IOHEN_R = crate::BitReader;
#[doc = "Field `IOHEN` writer - Enable for Port n"]
pub type IOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOHSRC` reader - Source for Port"]
pub type IOHSRC_R = crate::FieldReader;
#[doc = "Field `IOHSRC` writer - Source for Port"]
pub type IOHSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - CLK/CLK Enable for Port"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLK/CLK Source for Port"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DQS Enable for Port"]
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DQS Source for Port"]
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CS Enable for Port"]
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CS Source for Port"]
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable for Port"]
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Source for Port"]
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable for Port n"]
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Source for Port"]
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CLK/CLK Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<P2CRrs> {
        CLKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CLK/CLK Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<P2CRrs> {
        CLKSRC_W::new(self, 1)
    }
    #[doc = "Bit 4 - DQS Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn dqsen(&mut self) -> DQSEN_W<P2CRrs> {
        DQSEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - DQS Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn dqssrc(&mut self) -> DQSSRC_W<P2CRrs> {
        DQSSRC_W::new(self, 5)
    }
    #[doc = "Bit 8 - CS Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn ncsen(&mut self) -> NCSEN_W<P2CRrs> {
        NCSEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CS Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn ncssrc(&mut self) -> NCSSRC_W<P2CRrs> {
        NCSSRC_W::new(self, 9)
    }
    #[doc = "Bit 16 - Enable for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iolen(&mut self) -> IOLEN_W<P2CRrs> {
        IOLEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iolsrc(&mut self) -> IOLSRC_W<P2CRrs> {
        IOLSRC_W::new(self, 17)
    }
    #[doc = "Bit 24 - Enable for Port n"]
    #[inline(always)]
    #[must_use]
    pub fn iohen(&mut self) -> IOHEN_W<P2CRrs> {
        IOHEN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Source for Port"]
    #[inline(always)]
    #[must_use]
    pub fn iohsrc(&mut self) -> IOHSRC_W<P2CRrs> {
        IOHSRC_W::new(self, 25)
    }
}
#[doc = "OctoSPI IO Manager Port 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2CRrs;
impl crate::RegisterSpec for P2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2cr::R`](R) reader structure"]
impl crate::Readable for P2CRrs {}
#[doc = "`write(|w| ..)` method takes [`p2cr::W`](W) writer structure"]
impl crate::Writable for P2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P2CR to value 0x0705_0333"]
impl crate::Resettable for P2CRrs {
    const RESET_VALUE: u32 = 0x0705_0333;
}
