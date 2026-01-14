///Register `MMCRIMR` reader
pub type R = crate::R<MMCRIMRrs>;
///Register `MMCRIMR` writer
pub type W = crate::W<MMCRIMRrs>;
/**Received frame CRC error mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCEM {
    ///0: Received-crc-error counter half-full interrupt enabled
    Unmasked = 0,
    ///1: Received-crc-error counter half-full interrupt disabled
    Masked = 1,
}
impl From<RFCEM> for bool {
    #[inline(always)]
    fn from(variant: RFCEM) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCEM` reader - Received frame CRC error mask
pub type RFCEM_R = crate::BitReader<RFCEM>;
impl RFCEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFCEM {
        match self.bits {
            false => RFCEM::Unmasked,
            true => RFCEM::Masked,
        }
    }
    ///Received-crc-error counter half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFCEM::Unmasked
    }
    ///Received-crc-error counter half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFCEM::Masked
    }
}
///Field `RFCEM` writer - Received frame CRC error mask
pub type RFCEM_W<'a, REG> = crate::BitWriter<'a, REG, RFCEM>;
impl<'a, REG> RFCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Received-crc-error counter half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RFCEM::Unmasked)
    }
    ///Received-crc-error counter half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RFCEM::Masked)
    }
}
/**Received frames alignment error mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFAEM {
    ///0: Received-alignment-error counter half-full interrupt enabled
    Unmasked = 0,
    ///1: Received-alignment-error counter half-full interrupt disabled
    Masked = 1,
}
impl From<RFAEM> for bool {
    #[inline(always)]
    fn from(variant: RFAEM) -> Self {
        variant as u8 != 0
    }
}
///Field `RFAEM` reader - Received frames alignment error mask
pub type RFAEM_R = crate::BitReader<RFAEM>;
impl RFAEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFAEM {
        match self.bits {
            false => RFAEM::Unmasked,
            true => RFAEM::Masked,
        }
    }
    ///Received-alignment-error counter half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFAEM::Unmasked
    }
    ///Received-alignment-error counter half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFAEM::Masked
    }
}
///Field `RFAEM` writer - Received frames alignment error mask
pub type RFAEM_W<'a, REG> = crate::BitWriter<'a, REG, RFAEM>;
impl<'a, REG> RFAEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Received-alignment-error counter half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RFAEM::Unmasked)
    }
    ///Received-alignment-error counter half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RFAEM::Masked)
    }
}
/**Received good Unicast frames mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGUFM {
    ///0: Received-good-unicast counter half-full interrupt enabled
    Unmasked = 0,
    ///1: Received-good-unicast counter half-full interrupt disabled
    Masked = 1,
}
impl From<RGUFM> for bool {
    #[inline(always)]
    fn from(variant: RGUFM) -> Self {
        variant as u8 != 0
    }
}
///Field `RGUFM` reader - Received good Unicast frames mask
pub type RGUFM_R = crate::BitReader<RGUFM>;
impl RGUFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RGUFM {
        match self.bits {
            false => RGUFM::Unmasked,
            true => RGUFM::Masked,
        }
    }
    ///Received-good-unicast counter half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RGUFM::Unmasked
    }
    ///Received-good-unicast counter half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RGUFM::Masked
    }
}
///Field `RGUFM` writer - Received good Unicast frames mask
pub type RGUFM_W<'a, REG> = crate::BitWriter<'a, REG, RGUFM>;
impl<'a, REG> RGUFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Received-good-unicast counter half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RGUFM::Unmasked)
    }
    ///Received-good-unicast counter half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RGUFM::Masked)
    }
}
impl R {
    ///Bit 5 - Received frame CRC error mask
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Received frames alignment error mask
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - Received good Unicast frames mask
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRIMR")
            .field("rfcem", &self.rfcem())
            .field("rfaem", &self.rfaem())
            .field("rgufm", &self.rgufm())
            .finish()
    }
}
impl W {
    ///Bit 5 - Received frame CRC error mask
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W<'_, MMCRIMRrs> {
        RFCEM_W::new(self, 5)
    }
    ///Bit 6 - Received frames alignment error mask
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W<'_, MMCRIMRrs> {
        RFAEM_W::new(self, 6)
    }
    ///Bit 17 - Received good Unicast frames mask
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W<'_, MMCRIMRrs> {
        RGUFM_W::new(self, 17)
    }
}
/**Ethernet MMC receive interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmcrimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_MMC:MMCRIMR)*/
pub struct MMCRIMRrs;
impl crate::RegisterSpec for MMCRIMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmcrimr::R`](R) reader structure
impl crate::Readable for MMCRIMRrs {}
///`write(|w| ..)` method takes [`mmcrimr::W`](W) writer structure
impl crate::Writable for MMCRIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCRIMR to value 0
impl crate::Resettable for MMCRIMRrs {}
