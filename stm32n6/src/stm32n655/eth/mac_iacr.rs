///Register `MAC_IACR` reader
pub type R = crate::R<MAC_IACRrs>;
///Register `MAC_IACR` writer
pub type W = crate::W<MAC_IACRrs>;
///Field `OB` reader - Operation Busy.
pub type OB_R = crate::BitReader;
///Field `OB` writer - Operation Busy.
pub type OB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COM` reader - Command type
pub type COM_R = crate::BitReader;
///Field `COM` writer - Command type
pub type COM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTO` reader - Auto-increment
pub type AUTO_R = crate::BitReader;
///Field `AUTO` writer - Auto-increment
pub type AUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AOFF` reader - Address Offset
pub type AOFF_R = crate::FieldReader;
///Field `AOFF` writer - Address Offset
pub type AOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MSEL` reader - Mode Select
pub type MSEL_R = crate::FieldReader;
///Field `MSEL` writer - Mode Select
pub type MSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Operation Busy.
    #[inline(always)]
    pub fn ob(&self) -> OB_R {
        OB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command type
    #[inline(always)]
    pub fn com(&self) -> COM_R {
        COM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Auto-increment
    #[inline(always)]
    pub fn auto(&self) -> AUTO_R {
        AUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - Address Offset
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Mode Select
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_IACR")
            .field("ob", &self.ob())
            .field("com", &self.com())
            .field("auto", &self.auto())
            .field("aoff", &self.aoff())
            .field("msel", &self.msel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Operation Busy.
    #[inline(always)]
    pub fn ob(&mut self) -> OB_W<'_, MAC_IACRrs> {
        OB_W::new(self, 0)
    }
    ///Bit 1 - Command type
    #[inline(always)]
    pub fn com(&mut self) -> COM_W<'_, MAC_IACRrs> {
        COM_W::new(self, 1)
    }
    ///Bit 5 - Auto-increment
    #[inline(always)]
    pub fn auto(&mut self) -> AUTO_W<'_, MAC_IACRrs> {
        AUTO_W::new(self, 5)
    }
    ///Bits 8:15 - Address Offset
    #[inline(always)]
    pub fn aoff(&mut self) -> AOFF_W<'_, MAC_IACRrs> {
        AOFF_W::new(self, 8)
    }
    ///Bits 16:19 - Mode Select
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<'_, MAC_IACRrs> {
        MSEL_W::new(self, 16)
    }
}
/**MAC Indirect Access Control register

You can [`read`](crate::Reg::read) this register and get [`mac_iacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_iacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MAC_IACR)*/
pub struct MAC_IACRrs;
impl crate::RegisterSpec for MAC_IACRrs {
    type Ux = u32;
}
///`read()` method returns [`mac_iacr::R`](R) reader structure
impl crate::Readable for MAC_IACRrs {}
///`write(|w| ..)` method takes [`mac_iacr::W`](W) writer structure
impl crate::Writable for MAC_IACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAC_IACR to value 0
impl crate::Resettable for MAC_IACRrs {}
