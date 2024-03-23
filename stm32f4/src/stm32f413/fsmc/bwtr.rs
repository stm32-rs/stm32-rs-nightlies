#[doc = "Register `BWTR%s` reader"]
pub type R = crate::R<BWTRrs>;
#[doc = "Register `BWTR%s` writer"]
pub type W = crate::W<BWTRrs>;
#[doc = "Field `ADDSET` reader - Address setup phase duration"]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration"]
pub type ADDSET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration"]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration"]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - Data-phase duration"]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration"]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration"]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration"]
pub type BUSTURN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Access mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACCMOD {
    #[doc = "0: Access mode A"]
    A = 0,
    #[doc = "1: Access mode B"]
    B = 1,
    #[doc = "2: Access mode C"]
    C = 2,
    #[doc = "3: Access mode D"]
    D = 3,
}
impl From<ACCMOD> for u8 {
    #[inline(always)]
    fn from(variant: ACCMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACCMOD {
    type Ux = u8;
}
#[doc = "Field `ACCMOD` reader - Access mode"]
pub type ACCMOD_R = crate::FieldReader<ACCMOD>;
impl ACCMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACCMOD {
        match self.bits {
            0 => ACCMOD::A,
            1 => ACCMOD::B,
            2 => ACCMOD::C,
            3 => ACCMOD::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Access mode A"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == ACCMOD::A
    }
    #[doc = "Access mode B"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == ACCMOD::B
    }
    #[doc = "Access mode C"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == ACCMOD::C
    }
    #[doc = "Access mode D"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == ACCMOD::D
    }
}
#[doc = "Field `ACCMOD` writer - Access mode"]
pub type ACCMOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ACCMOD>;
impl<'a, REG> ACCMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access mode A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::A)
    }
    #[doc = "Access mode B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::B)
    }
    #[doc = "Access mode C"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::C)
    }
    #[doc = "Access mode D"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::D)
    }
}
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BWTRrs> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTRrs> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BWTRrs> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BWTRrs> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTRrs> {
        ACCMOD_W::new(self, 28)
    }
}
#[doc = "BWTR%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BWTRrs;
impl crate::RegisterSpec for BWTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bwtr::R`](R) reader structure"]
impl crate::Readable for BWTRrs {}
#[doc = "`write(|w| ..)` method takes [`bwtr::W`](W) writer structure"]
impl crate::Writable for BWTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BWTR%s to value 0"]
impl crate::Resettable for BWTRrs {
    const RESET_VALUE: u32 = 0;
}
