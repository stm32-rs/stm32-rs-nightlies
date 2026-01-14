///Register `TBR` reader
pub type R = crate::R<TBRrs>;
///Register `TBR` writer
pub type W = crate::W<TBRrs>;
///Field `TSEL` reader - Trigger selection
pub type TSEL_R = crate::FieldReader;
///Field `TSEL` writer - Trigger selection
pub type TSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0.
pub type SBUS_R = crate::BitReader;
///Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0.
pub type SBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0.
pub type DBUS_R = crate::BitReader;
///Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0.
pub type DBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Trigger selection
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBR")
            .field("tsel", &self.tsel())
            .field("sbus", &self.sbus())
            .field("dbus", &self.dbus())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Trigger selection
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W<'_, TBRrs> {
        TSEL_W::new(self, 0)
    }
    ///Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W<'_, TBRrs> {
        SBUS_W::new(self, 16)
    }
    ///Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0.
    #[inline(always)]
    pub fn dbus(&mut self) -> DBUS_W<'_, TBRrs> {
        DBUS_W::new(self, 17)
    }
}
/**MDMA channel x Trigger and Bus selection Register

You can [`read`](crate::Reg::read) this register and get [`tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TBRrs;
impl crate::RegisterSpec for TBRrs {
    type Ux = u32;
}
///`read()` method returns [`tbr::R`](R) reader structure
impl crate::Readable for TBRrs {}
///`write(|w| ..)` method takes [`tbr::W`](W) writer structure
impl crate::Writable for TBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TBR to value 0
impl crate::Resettable for TBRrs {}
