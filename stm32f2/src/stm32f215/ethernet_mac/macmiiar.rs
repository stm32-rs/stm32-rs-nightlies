#[doc = "Register `MACMIIAR` reader"]
pub type R = crate::R<MACMIIARrs>;
#[doc = "Register `MACMIIAR` writer"]
pub type W = crate::W<MACMIIARrs>;
#[doc = "Field `MB` reader - MII busy"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII busy"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` reader - MII write"]
pub type MW_R = crate::BitReader;
#[doc = "Field `MW` writer - MII write"]
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - Clock range"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - Clock range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MR` reader - MII register"]
pub type MR_R = crate::FieldReader;
#[doc = "Field `MR` writer - MII register"]
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MACMIIARrs> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    #[must_use]
    pub fn mw(&mut self) -> MW_W<MACMIIARrs> {
        MW_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MACMIIARrs> {
        CR_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<MACMIIARrs> {
        MR_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MACMIIARrs> {
        PA_W::new(self, 11)
    }
}
#[doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIARrs;
impl crate::RegisterSpec for MACMIIARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiiar::R`](R) reader structure"]
impl crate::Readable for MACMIIARrs {}
#[doc = "`write(|w| ..)` method takes [`macmiiar::W`](W) writer structure"]
impl crate::Writable for MACMIIARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMIIAR to value 0"]
impl crate::Resettable for MACMIIARrs {
    const RESET_VALUE: u32 = 0;
}
