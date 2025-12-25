///Register `CSR1` reader
pub type R = crate::R<CSR1rs>;
///Register `CSR1` writer
pub type W = crate::W<CSR1rs>;
///Field `BREN` reader - Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes.
pub type BREN_R = crate::BitReader;
///Field `BREN` writer - Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes.
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONEN` reader - V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1).
pub type MONEN_R = crate::BitReader;
///Field `MONEN` writer - V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1).
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRRDY` reader - Backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.
pub type BRRDY_R = crate::BitReader;
///Field `VBATL` reader - V<sub>BAT</sub> level monitoring versus low threshold
pub type VBATL_R = crate::BitReader;
///Field `VBATH` reader - V<sub>BAT</sub> level monitoring versus high threshold
pub type VBATH_R = crate::BitReader;
///Field `TEMPL` reader - Temperature level monitoring versus low threshold
pub type TEMPL_R = crate::BitReader;
///Field `TEMPH` reader - Temperature level monitoring versus high threshold
pub type TEMPH_R = crate::BitReader;
impl R {
    ///Bit 0 - Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes.
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1).
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - Backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - V<sub>BAT</sub> level monitoring versus low threshold
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - V<sub>BAT</sub> level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR1")
            .field("bren", &self.bren())
            .field("monen", &self.monen())
            .field("brrdy", &self.brrdy())
            .field("vbatl", &self.vbatl())
            .field("vbath", &self.vbath())
            .field("templ", &self.templ())
            .field("temph", &self.temph())
            .finish()
    }
}
impl W {
    ///Bit 0 - Backup regulator enable When set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and V<sub>BAT</sub> modes.
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<'_, CSR1rs> {
        BREN_W::new(self, 0)
    }
    ///Bit 4 - V<sub>BAT</sub> and temperature monitoring enable When set, the V<sub>BAT</sub> supply and temperature monitoring is enabled. Note: V<sub>BAT</sub> and temperature monitoring are only available when the backup regulator is enabled (BREN bit set to 1).
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W<'_, CSR1rs> {
        MONEN_W::new(self, 4)
    }
}
/**PWR control status register 1

You can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#PWR:CSR1)*/
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`csr1::R`](R) reader structure
impl crate::Readable for CSR1rs {}
///`write(|w| ..)` method takes [`csr1::W`](W) writer structure
impl crate::Writable for CSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR1 to value 0
impl crate::Resettable for CSR1rs {}
