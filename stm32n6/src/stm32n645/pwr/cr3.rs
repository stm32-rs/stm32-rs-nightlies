///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `VCOREMONEN` reader - V less than sub>DDCORE less than /sub> monitoring enable
pub type VCOREMONEN_R = crate::BitReader;
///Field `VCOREMONEN` writer - V less than sub>DDCORE less than /sub> monitoring enable
pub type VCOREMONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCORELLS` reader - V less than sub>DDCORE less than /sub> voltage detector low-level selection
pub type VCORELLS_R = crate::BitReader;
///Field `VCORELLS` writer - V less than sub>DDCORE less than /sub> voltage detector low-level selection
pub type VCORELLS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCOREL` reader - Monitored V less than sub>DDCORE less than /sub> level above low threshold
pub type VCOREL_R = crate::BitReader;
///Field `VCOREH` reader - Monitored V less than sub>DDCORE less than /sub> level above high threshold
pub type VCOREH_R = crate::BitReader;
impl R {
    ///Bit 0 - V less than sub>DDCORE less than /sub> monitoring enable
    #[inline(always)]
    pub fn vcoremonen(&self) -> VCOREMONEN_R {
        VCOREMONEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - V less than sub>DDCORE less than /sub> voltage detector low-level selection
    #[inline(always)]
    pub fn vcorells(&self) -> VCORELLS_R {
        VCORELLS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Monitored V less than sub>DDCORE less than /sub> level above low threshold
    #[inline(always)]
    pub fn vcorel(&self) -> VCOREL_R {
        VCOREL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Monitored V less than sub>DDCORE less than /sub> level above high threshold
    #[inline(always)]
    pub fn vcoreh(&self) -> VCOREH_R {
        VCOREH_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("vcoremonen", &self.vcoremonen())
            .field("vcorells", &self.vcorells())
            .field("vcorel", &self.vcorel())
            .field("vcoreh", &self.vcoreh())
            .finish()
    }
}
impl W {
    ///Bit 0 - V less than sub>DDCORE less than /sub> monitoring enable
    #[inline(always)]
    pub fn vcoremonen(&mut self) -> VCOREMONEN_W<'_, CR3rs> {
        VCOREMONEN_W::new(self, 0)
    }
    ///Bit 4 - V less than sub>DDCORE less than /sub> voltage detector low-level selection
    #[inline(always)]
    pub fn vcorells(&mut self) -> VCORELLS_W<'_, CR3rs> {
        VCORELLS_W::new(self, 4)
    }
}
/**PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
