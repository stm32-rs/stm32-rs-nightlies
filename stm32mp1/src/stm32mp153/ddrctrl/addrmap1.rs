///Register `ADDRMAP1` reader
pub type R = crate::R<ADDRMAP1rs>;
///Register `ADDRMAP1` writer
pub type W = crate::W<ADDRMAP1rs>;
///Field `ADDRMAP_BANK_B0` reader - ADDRMAP_BANK_B0
pub type ADDRMAP_BANK_B0_R = crate::FieldReader;
///Field `ADDRMAP_BANK_B0` writer - ADDRMAP_BANK_B0
pub type ADDRMAP_BANK_B0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ADDRMAP_BANK_B1` reader - ADDRMAP_BANK_B1
pub type ADDRMAP_BANK_B1_R = crate::FieldReader;
///Field `ADDRMAP_BANK_B1` writer - ADDRMAP_BANK_B1
pub type ADDRMAP_BANK_B1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ADDRMAP_BANK_B2` reader - ADDRMAP_BANK_B2
pub type ADDRMAP_BANK_B2_R = crate::FieldReader;
///Field `ADDRMAP_BANK_B2` writer - ADDRMAP_BANK_B2
pub type ADDRMAP_BANK_B2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - ADDRMAP_BANK_B0
    #[inline(always)]
    pub fn addrmap_bank_b0(&self) -> ADDRMAP_BANK_B0_R {
        ADDRMAP_BANK_B0_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - ADDRMAP_BANK_B1
    #[inline(always)]
    pub fn addrmap_bank_b1(&self) -> ADDRMAP_BANK_B1_R {
        ADDRMAP_BANK_B1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - ADDRMAP_BANK_B2
    #[inline(always)]
    pub fn addrmap_bank_b2(&self) -> ADDRMAP_BANK_B2_R {
        ADDRMAP_BANK_B2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP1")
            .field("addrmap_bank_b0", &self.addrmap_bank_b0())
            .field("addrmap_bank_b1", &self.addrmap_bank_b1())
            .field("addrmap_bank_b2", &self.addrmap_bank_b2())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - ADDRMAP_BANK_B0
    #[inline(always)]
    pub fn addrmap_bank_b0(&mut self) -> ADDRMAP_BANK_B0_W<'_, ADDRMAP1rs> {
        ADDRMAP_BANK_B0_W::new(self, 0)
    }
    ///Bits 8:13 - ADDRMAP_BANK_B1
    #[inline(always)]
    pub fn addrmap_bank_b1(&mut self) -> ADDRMAP_BANK_B1_W<'_, ADDRMAP1rs> {
        ADDRMAP_BANK_B1_W::new(self, 8)
    }
    ///Bits 16:21 - ADDRMAP_BANK_B2
    #[inline(always)]
    pub fn addrmap_bank_b2(&mut self) -> ADDRMAP_BANK_B2_W<'_, ADDRMAP1rs> {
        ADDRMAP_BANK_B2_W::new(self, 16)
    }
}
/**DDRCTRL address map register 1

You can [`read`](crate::Reg::read) this register and get [`addrmap1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:ADDRMAP1)*/
pub struct ADDRMAP1rs;
impl crate::RegisterSpec for ADDRMAP1rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap1::R`](R) reader structure
impl crate::Readable for ADDRMAP1rs {}
///`write(|w| ..)` method takes [`addrmap1::W`](W) writer structure
impl crate::Writable for ADDRMAP1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP1 to value 0
impl crate::Resettable for ADDRMAP1rs {}
