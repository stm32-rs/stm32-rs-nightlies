///Register `ESR` reader
pub type R = crate::R<ESRrs>;
///Register `ESR` writer
pub type W = crate::W<ESRrs>;
///Field `EWGF` reader - EWGF
pub type EWGF_R = crate::BitReader;
///Field `EPVF` reader - EPVF
pub type EPVF_R = crate::BitReader;
///Field `BOFF` reader - BOFF
pub type BOFF_R = crate::BitReader;
/**LEC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC {
    ///0: No Error
    NoError = 0,
    ///1: Stuff Error
    Stuff = 1,
    ///2: Form Error
    Form = 2,
    ///3: Acknowledgment Error
    Ack = 3,
    ///4: Bit recessive Error
    BitRecessive = 4,
    ///5: Bit dominant Error
    BitDominant = 5,
    ///6: CRC Error
    Crc = 6,
    ///7: Set by software
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
impl crate::IsEnum for LEC {}
///Field `LEC` reader - LEC
pub type LEC_R = crate::FieldReader<LEC>;
impl LEC_R {
    ///Get enumerated values variant
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
    ///No Error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC::NoError
    }
    ///Stuff Error
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC::Stuff
    }
    ///Form Error
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC::Form
    }
    ///Acknowledgment Error
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC::Ack
    }
    ///Bit recessive Error
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == LEC::BitRecessive
    }
    ///Bit dominant Error
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == LEC::BitDominant
    }
    ///CRC Error
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC::Crc
    }
    ///Set by software
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == LEC::Custom
    }
}
///Field `LEC` writer - LEC
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LEC, crate::Safe>;
impl<'a, REG> LEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No Error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::NoError)
    }
    ///Stuff Error
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Stuff)
    }
    ///Form Error
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Form)
    }
    ///Acknowledgment Error
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Ack)
    }
    ///Bit recessive Error
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::BitRecessive)
    }
    ///Bit dominant Error
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::BitDominant)
    }
    ///CRC Error
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Crc)
    }
    ///Set by software
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::Custom)
    }
}
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - REC
pub type REC_R = crate::FieldReader;
impl R {
    ///Bit 0 - EWGF
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPVF
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BOFF
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - LEC
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 16:23 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - REC
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESR")
            .field("rec", &self.rec())
            .field("tec", &self.tec())
            .field("lec", &self.lec())
            .field("boff", &self.boff())
            .field("epvf", &self.epvf())
            .field("ewgf", &self.ewgf())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - LEC
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<'_, ESRrs> {
        LEC_W::new(self, 4)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`esr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#CAN1:ESR)*/
pub struct ESRrs;
impl crate::RegisterSpec for ESRrs {
    type Ux = u32;
}
///`read()` method returns [`esr::R`](R) reader structure
impl crate::Readable for ESRrs {}
///`write(|w| ..)` method takes [`esr::W`](W) writer structure
impl crate::Writable for ESRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESR to value 0
impl crate::Resettable for ESRrs {}
