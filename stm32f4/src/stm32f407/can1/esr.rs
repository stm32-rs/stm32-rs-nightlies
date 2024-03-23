#[doc = "Register `ESR` reader"]
pub type R = crate::R<ESRrs>;
#[doc = "Register `ESR` writer"]
pub type W = crate::W<ESRrs>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader;
#[doc = "LEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC {
    #[doc = "0: No Error"]
    NoError = 0,
    #[doc = "1: Stuff Error"]
    Stuff = 1,
    #[doc = "2: Form Error"]
    Form = 2,
    #[doc = "3: Acknowledgment Error"]
    Ack = 3,
    #[doc = "4: Bit recessive Error"]
    BitRecessive = 4,
    #[doc = "5: Bit dominant Error"]
    BitDominant = 5,
    #[doc = "6: CRC Error"]
    Crc = 6,
    #[doc = "7: Set by software"]
    Custom = 7,
}
impl From<LEC> for u8 {
    #[inline(always)]
    fn from(variant: LEC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEC {
    type Ux = u8;
}
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<LEC>;
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEC {
        match self.bits {
            0 => LEC::NoError,
            1 => LEC::Stuff,
            2 => LEC::Form,
            3 => LEC::Ack,
            4 => LEC::BitRecessive,
            5 => LEC::BitDominant,
            6 => LEC::Crc,
            7 => LEC::Custom,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC::NoError
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC::Stuff
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC::Form
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC::Ack
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == LEC::BitRecessive
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == LEC::BitDominant
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC::Crc
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == LEC::Custom
    }
}
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LEC>;
impl<'a, REG> LEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::NoError)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Stuff)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Form)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Ack)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::BitRecessive)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::BitDominant)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Crc)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Custom)
    }
}
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<ESRrs> {
        LEC_W::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESRrs;
impl crate::RegisterSpec for ESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esr::R`](R) reader structure"]
impl crate::Readable for ESRrs {}
#[doc = "`write(|w| ..)` method takes [`esr::W`](W) writer structure"]
impl crate::Writable for ESRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESRrs {
    const RESET_VALUE: u32 = 0;
}
