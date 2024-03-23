#[doc = "Register `OAR1` reader"]
pub type R = crate::R<OAR1rs>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<OAR1rs>;
#[doc = "Field `ADD` reader - Interface address"]
pub type ADD_R = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - Interface address"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Addressing mode (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDMODE {
    #[doc = "0: 7-bit slave address"]
    Add7 = 0,
    #[doc = "1: 10-bit slave address"]
    Add10 = 1,
}
impl From<ADDMODE> for bool {
    #[inline(always)]
    fn from(variant: ADDMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type ADDMODE_R = crate::BitReader<ADDMODE>;
impl ADDMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDMODE {
        match self.bits {
            false => ADDMODE::Add7,
            true => ADDMODE::Add10,
        }
    }
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDMODE::Add7
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDMODE::Add10
    }
}
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type ADDMODE_W<'a, REG> = crate::BitWriter<'a, REG, ADDMODE>;
impl<'a, REG> ADDMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDMODE::Add7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut crate::W<REG> {
        self.variant(ADDMODE::Add10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<OAR1rs> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> ADDMODE_W<OAR1rs> {
        ADDMODE_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for OAR1rs {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1rs {
    const RESET_VALUE: u32 = 0;
}
