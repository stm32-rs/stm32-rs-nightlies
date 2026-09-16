///Register `C19TBR` reader
pub type R = crate::R<C19TBRrs>;
///Register `C19TBR` writer
pub type W = crate::W<C19TBRrs>;
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
        f.debug_struct("C19TBR")
            .field("tsel", &self.tsel())
            .field("sbus", &self.sbus())
            .field("dbus", &self.dbus())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - TSEL
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W<'_, C19TBRrs> {
        TSEL_W::new(self, 0)
    }
    ///Bit 16 - SBUS
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W<'_, C19TBRrs> {
        SBUS_W::new(self, 16)
    }
    ///Bit 17 - DBUS
    #[inline(always)]
    pub fn dbus(&mut self) -> DBUS_W<'_, C19TBRrs> {
        DBUS_W::new(self, 17)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c19tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C19TBR)*/
pub struct C19TBRrs;
impl crate::RegisterSpec for C19TBRrs {
    type Ux = u32;
}
///`read()` method returns [`c19tbr::R`](R) reader structure
impl crate::Readable for C19TBRrs {}
///`write(|w| ..)` method takes [`c19tbr::W`](W) writer structure
impl crate::Writable for C19TBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C19TBR to value 0
impl crate::Resettable for C19TBRrs {}
