#[doc = "Register `FMC_CSQAR2` reader"]
pub type R = crate::R<FMC_CSQAR2rs>;
#[doc = "Register `FMC_CSQAR2` writer"]
pub type W = crate::W<FMC_CSQAR2rs>;
#[doc = "Field `ADDC5` reader - ADDC5"]
pub type ADDC5_R = crate::FieldReader;
#[doc = "Field `ADDC5` writer - ADDC5"]
pub type ADDC5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NANDCEN0` reader - NANDCEN0"]
pub type NANDCEN0_R = crate::BitReader;
#[doc = "Field `NANDCEN0` writer - NANDCEN0"]
pub type NANDCEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NANDCEN1` reader - NANDCEN1"]
pub type NANDCEN1_R = crate::BitReader;
#[doc = "Field `NANDCEN1` writer - NANDCEN1"]
pub type NANDCEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAO` reader - SAO"]
pub type SAO_R = crate::FieldReader<u16>;
#[doc = "Field `SAO` writer - SAO"]
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&self) -> NANDCEN0_R {
        NANDCEN0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&self) -> NANDCEN1_R {
        NANDCEN1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    #[must_use]
    pub fn addc5(&mut self) -> ADDC5_W<FMC_CSQAR2rs> {
        ADDC5_W::new(self, 0)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    #[must_use]
    pub fn nandcen0(&mut self) -> NANDCEN0_W<FMC_CSQAR2rs> {
        NANDCEN0_W::new(self, 10)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    #[must_use]
    pub fn nandcen1(&mut self) -> NANDCEN1_W<FMC_CSQAR2rs> {
        NANDCEN1_W::new(self, 11)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    #[must_use]
    pub fn sao(&mut self) -> SAO_W<FMC_CSQAR2rs> {
        SAO_W::new(self, 16)
    }
}
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQAR2rs;
impl crate::RegisterSpec for FMC_CSQAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqar2::R`](R) reader structure"]
impl crate::Readable for FMC_CSQAR2rs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqar2::W`](W) writer structure"]
impl crate::Writable for FMC_CSQAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQAR2 to value 0x0002_0000"]
impl crate::Resettable for FMC_CSQAR2rs {
    const RESET_VALUE: u32 = 0x0002_0000;
}
