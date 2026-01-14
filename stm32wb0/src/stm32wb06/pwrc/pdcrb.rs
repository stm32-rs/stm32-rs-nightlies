///Register `PDCRB` reader
pub type R = crate::R<PDCRBrs>;
///Register `PDCRB` writer
pub type W = crate::W<PDCRBrs>;
///Field `PDB` reader - PDB\[x\]: Pull Down Pull Down activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PDB_R = crate::FieldReader<u16>;
///Field `PDB` writer - PDB\[x\]: Pull Down Pull Down activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PDB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PDB\[x\]: Pull Down Pull Down activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRB").field("pdb", &self.pdb()).finish()
    }
}
impl W {
    ///Bits 0:15 - PDB\[x\]: Pull Down Pull Down activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pdb(&mut self) -> PDB_W<'_, PDCRBrs> {
        PDB_W::new(self, 0)
    }
}
/**PDCRB register

You can [`read`](crate::Reg::read) this register and get [`pdcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:PDCRB)*/
pub struct PDCRBrs;
impl crate::RegisterSpec for PDCRBrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrb::R`](R) reader structure
impl crate::Readable for PDCRBrs {}
///`write(|w| ..)` method takes [`pdcrb::W`](W) writer structure
impl crate::Writable for PDCRBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRB to value 0
impl crate::Resettable for PDCRBrs {}
