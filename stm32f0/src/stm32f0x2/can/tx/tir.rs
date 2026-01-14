///Register `TIR` reader
pub type R = crate::R<TIRrs>;
///Register `TIR` writer
pub type W = crate::W<TIRrs>;
///Field `TXRQ` reader - TXRQ
pub type TXRQ_R = crate::BitReader;
///Field `TXRQ` writer - TXRQ
pub type TXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
/**RTR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR {
    ///0: Data frame
    Data = 0,
    ///1: Remote frame
    Remote = 1,
}
impl From<RTR> for bool {
    #[inline(always)]
    fn from(variant: RTR) -> Self {
        variant as u8 != 0
    }
}
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader<RTR>;
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTR {
        match self.bits {
            false => RTR::Data,
            true => RTR::Remote,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR::Data
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR::Remote
    }
}
///Field `RTR` writer - RTR
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG, RTR>;
impl<'a, REG> RTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data frame
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(RTR::Data)
    }
    ///Remote frame
    #[inline(always)]
    pub fn remote(self) -> &'a mut crate::W<REG> {
        self.variant(RTR::Remote)
    }
}
/**IDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE {
    ///0: Standard identifier
    Standard = 0,
    ///1: Extended identifier
    Extended = 1,
}
impl From<IDE> for bool {
    #[inline(always)]
    fn from(variant: IDE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader<IDE>;
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDE {
        match self.bits {
            false => IDE::Standard,
            true => IDE::Extended,
        }
    }
    ///Standard identifier
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE::Standard
    }
    ///Extended identifier
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE::Extended
    }
}
///Field `IDE` writer - IDE
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG, IDE>;
impl<'a, REG> IDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard identifier
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(IDE::Standard)
    }
    ///Extended identifier
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(IDE::Extended)
    }
}
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32>;
///Field `EXID` writer - EXID
pub type EXID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16>;
///Field `STID` writer - STID
pub type STID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIR")
            .field("stid", &self.stid())
            .field("exid", &self.exid())
            .field("ide", &self.ide())
            .field("rtr", &self.rtr())
            .field("txrq", &self.txrq())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W<'_, TIRrs> {
        TXRQ_W::new(self, 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W<'_, TIRrs> {
        RTR_W::new(self, 1)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W<'_, TIRrs> {
        IDE_W::new(self, 2)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W<'_, TIRrs> {
        EXID_W::new(self, 3)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W<'_, TIRrs> {
        STID_W::new(self, 21)
    }
}
/**CAN_TI0R

You can [`read`](crate::Reg::read) this register and get [`tir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIRrs;
impl crate::RegisterSpec for TIRrs {
    type Ux = u32;
}
///`read()` method returns [`tir::R`](R) reader structure
impl crate::Readable for TIRrs {}
///`write(|w| ..)` method takes [`tir::W`](W) writer structure
impl crate::Writable for TIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIR to value 0
impl crate::Resettable for TIRrs {}
