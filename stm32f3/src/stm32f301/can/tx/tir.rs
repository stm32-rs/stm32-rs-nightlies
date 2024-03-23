#[doc = "Register `TIR` reader"]
pub type R = crate::R<TIRrs>;
#[doc = "Register `TIR` writer"]
pub type W = crate::W<TIRrs>;
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<RTR> for bool {
    #[inline(always)]
    fn from(variant: RTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<RTR>;
impl RTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTR {
        match self.bits {
            false => RTR::Data,
            true => RTR::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR::Remote
    }
}
#[doc = "Field `RTR` writer - RTR"]
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG, RTR>;
impl<'a, REG> RTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(RTR::Data)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut crate::W<REG> {
        self.variant(RTR::Remote)
    }
}
#[doc = "IDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE {
    #[doc = "0: Standard identifier"]
    Standard = 0,
    #[doc = "1: Extended identifier"]
    Extended = 1,
}
impl From<IDE> for bool {
    #[inline(always)]
    fn from(variant: IDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<IDE>;
impl IDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDE {
        match self.bits {
            false => IDE::Standard,
            true => IDE::Extended,
        }
    }
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE::Standard
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE::Extended
    }
}
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG, IDE>;
impl<'a, REG> IDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(IDE::Standard)
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(IDE::Extended)
    }
}
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type EXID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16>;
#[doc = "Field `STID` writer - STID"]
pub type STID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    #[must_use]
    pub fn txrq(&mut self) -> TXRQ_W<TIRrs> {
        TXRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<TIRrs> {
        RTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<TIRrs> {
        IDE_W::new(self, 2)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    #[must_use]
    pub fn exid(&mut self) -> EXID_W<TIRrs> {
        EXID_W::new(self, 3)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    #[must_use]
    pub fn stid(&mut self) -> STID_W<TIRrs> {
        STID_W::new(self, 21)
    }
}
#[doc = "TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIRrs;
impl crate::RegisterSpec for TIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tir::R`](R) reader structure"]
impl crate::Readable for TIRrs {}
#[doc = "`write(|w| ..)` method takes [`tir::W`](W) writer structure"]
impl crate::Writable for TIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIR to value 0"]
impl crate::Resettable for TIRrs {
    const RESET_VALUE: u32 = 0;
}
