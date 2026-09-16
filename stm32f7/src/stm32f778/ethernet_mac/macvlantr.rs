///Register `MACVLANTR` reader
pub type R = crate::R<MACVLANTRrs>;
///Register `MACVLANTR` writer
pub type W = crate::W<MACVLANTRrs>;
///Field `VLANTI` reader - VLAN tag identifier (for receive frames)
pub type VLANTI_R = crate::FieldReader<u16>;
///Field `VLANTI` writer - VLAN tag identifier (for receive frames)
pub type VLANTI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
/**12-bit VLAN tag comparison

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLANTC {
    ///0: Full 16 bit VLAN identifiers are used for comparison and filtering
    Vlantc16 = 0,
    ///1: 12 bit VLAN identifies are used for comparison and filtering
    Vlantc12 = 1,
}
impl From<VLANTC> for bool {
    #[inline(always)]
    fn from(variant: VLANTC) -> Self {
        variant as u8 != 0
    }
}
///Field `VLANTC` reader - 12-bit VLAN tag comparison
pub type VLANTC_R = crate::BitReader<VLANTC>;
impl VLANTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VLANTC {
        match self.bits {
            false => VLANTC::Vlantc16,
            true => VLANTC::Vlantc12,
        }
    }
    ///Full 16 bit VLAN identifiers are used for comparison and filtering
    #[inline(always)]
    pub fn is_vlantc16(&self) -> bool {
        *self == VLANTC::Vlantc16
    }
    ///12 bit VLAN identifies are used for comparison and filtering
    #[inline(always)]
    pub fn is_vlantc12(&self) -> bool {
        *self == VLANTC::Vlantc12
    }
}
///Field `VLANTC` writer - 12-bit VLAN tag comparison
pub type VLANTC_W<'a, REG> = crate::BitWriter<'a, REG, VLANTC>;
impl<'a, REG> VLANTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full 16 bit VLAN identifiers are used for comparison and filtering
    #[inline(always)]
    pub fn vlantc16(self) -> &'a mut crate::W<REG> {
        self.variant(VLANTC::Vlantc16)
    }
    ///12 bit VLAN identifies are used for comparison and filtering
    #[inline(always)]
    pub fn vlantc12(self) -> &'a mut crate::W<REG> {
        self.variant(VLANTC::Vlantc12)
    }
}
impl R {
    ///Bits 0:15 - VLAN tag identifier (for receive frames)
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - 12-bit VLAN tag comparison
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVLANTR")
            .field("vlanti", &self.vlanti())
            .field("vlantc", &self.vlantc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLAN tag identifier (for receive frames)
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W<'_, MACVLANTRrs> {
        VLANTI_W::new(self, 0)
    }
    ///Bit 16 - 12-bit VLAN tag comparison
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W<'_, MACVLANTRrs> {
        VLANTC_W::new(self, 16)
    }
}
/**Ethernet MAC VLAN tag register

You can [`read`](crate::Reg::read) this register and get [`macvlantr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlantr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#Ethernet_MAC:MACVLANTR)*/
pub struct MACVLANTRrs;
impl crate::RegisterSpec for MACVLANTRrs {
    type Ux = u32;
}
///`read()` method returns [`macvlantr::R`](R) reader structure
impl crate::Readable for MACVLANTRrs {}
///`write(|w| ..)` method takes [`macvlantr::W`](W) writer structure
impl crate::Writable for MACVLANTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVLANTR to value 0
impl crate::Resettable for MACVLANTRrs {}
