///Register `C3TBR` reader
pub type R = crate::R<C3TBRrs>;
///Register `C3TBR` writer
pub type W = crate::W<C3TBRrs>;
///Field `TSEL` reader - TSEL
pub type TSEL_R = crate::FieldReader;
///Field `TSEL` writer - TSEL
pub type TSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SBUS` reader - SBUS
pub type SBUS_R = crate::BitReader;
///Field `SBUS` writer - SBUS
pub type SBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBUS` reader - DBUS
pub type DBUS_R = crate::BitReader;
///Field `DBUS` writer - DBUS
pub type DBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - TSEL
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - SBUS
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DBUS
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3TBR")
            .field("tsel", &self.tsel())
            .field("sbus", &self.sbus())
            .field("dbus", &self.dbus())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - TSEL
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W<'_, C3TBRrs> {
        TSEL_W::new(self, 0)
    }
    ///Bit 16 - SBUS
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W<'_, C3TBRrs> {
        SBUS_W::new(self, 16)
    }
    ///Bit 17 - DBUS
    #[inline(always)]
    pub fn dbus(&mut self) -> DBUS_W<'_, C3TBRrs> {
        DBUS_W::new(self, 17)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c3tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3TBR)*/
pub struct C3TBRrs;
impl crate::RegisterSpec for C3TBRrs {
    type Ux = u32;
}
///`read()` method returns [`c3tbr::R`](R) reader structure
impl crate::Readable for C3TBRrs {}
///`write(|w| ..)` method takes [`c3tbr::W`](W) writer structure
impl crate::Writable for C3TBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3TBR to value 0
impl crate::Resettable for C3TBRrs {}
