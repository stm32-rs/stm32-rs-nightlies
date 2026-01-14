///Register `MACA1HR` reader
pub type R = crate::R<MACA1HRrs>;
///Register `MACA1HR` writer
pub type W = crate::W<MACA1HRrs>;
///Field `MACA1H` reader - MACA1H
pub type MACA1H_R = crate::FieldReader<u16>;
///Field `MACA1H` writer - MACA1H
pub type MACA1H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `MBC` reader - MBC
pub type MBC_R = crate::FieldReader;
///Field `MBC` writer - MBC
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
/**SA

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SA {
    ///0: This address is used for comparison with DA fields of the received frame
    Destination = 0,
    ///1: This address is used for comparison with SA fields of received frames
    Source = 1,
}
impl From<SA> for bool {
    #[inline(always)]
    fn from(variant: SA) -> Self {
        variant as u8 != 0
    }
}
///Field `SA` reader - SA
pub type SA_R = crate::BitReader<SA>;
impl SA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SA {
        match self.bits {
            false => SA::Destination,
            true => SA::Source,
        }
    }
    ///This address is used for comparison with DA fields of the received frame
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == SA::Destination
    }
    ///This address is used for comparison with SA fields of received frames
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SA::Source
    }
}
///Field `SA` writer - SA
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG, SA>;
impl<'a, REG> SA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This address is used for comparison with DA fields of the received frame
    #[inline(always)]
    pub fn destination(self) -> &'a mut crate::W<REG> {
        self.variant(SA::Destination)
    }
    ///This address is used for comparison with SA fields of received frames
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(SA::Source)
    }
}
/**AE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AE {
    ///0: Address filters ignore this address
    Disabled = 0,
    ///1: Address filters use this address
    Enabled = 1,
}
impl From<AE> for bool {
    #[inline(always)]
    fn from(variant: AE) -> Self {
        variant as u8 != 0
    }
}
///Field `AE` reader - AE
pub type AE_R = crate::BitReader<AE>;
impl AE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AE {
        match self.bits {
            false => AE::Disabled,
            true => AE::Enabled,
        }
    }
    ///Address filters ignore this address
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AE::Disabled
    }
    ///Address filters use this address
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AE::Enabled
    }
}
///Field `AE` writer - AE
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG, AE>;
impl<'a, REG> AE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address filters ignore this address
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AE::Disabled)
    }
    ///Address filters use this address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AE::Enabled)
    }
}
impl R {
    ///Bits 0:15 - MACA1H
    #[inline(always)]
    pub fn maca1h(&self) -> MACA1H_R {
        MACA1H_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1HR")
            .field("maca1h", &self.maca1h())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - MACA1H
    #[inline(always)]
    pub fn maca1h(&mut self) -> MACA1H_W<'_, MACA1HRrs> {
        MACA1H_W::new(self, 0)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<'_, MACA1HRrs> {
        MBC_W::new(self, 24)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, MACA1HRrs> {
        SA_W::new(self, 30)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<'_, MACA1HRrs> {
        AE_W::new(self, 31)
    }
}
/**Ethernet MAC address 1 high register

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#Ethernet_MAC:MACA1HR)*/
pub struct MACA1HRrs;
impl crate::RegisterSpec for MACA1HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca1hr::R`](R) reader structure
impl crate::Readable for MACA1HRrs {}
///`write(|w| ..)` method takes [`maca1hr::W`](W) writer structure
impl crate::Writable for MACA1HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA1HR to value 0xffff
impl crate::Resettable for MACA1HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
