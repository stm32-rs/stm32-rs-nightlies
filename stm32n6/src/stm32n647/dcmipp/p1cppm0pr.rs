///Register `P1CPPM0PR` reader
pub type R = crate::R<P1CPPM0PRrs>;
///Field `PITCH` reader - Current number of bytes between the address of two consecutive lines.
pub type PITCH_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Current number of bytes between the address of two consecutive lines.
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CPPM0PR")
            .field("pitch", &self.pitch())
            .finish()
    }
}
/**DCMIPP Pipex current pixel packer Memory0 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1cppm0pr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CPPM0PR)*/
pub struct P1CPPM0PRrs;
impl crate::RegisterSpec for P1CPPM0PRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cppm0pr::R`](R) reader structure
impl crate::Readable for P1CPPM0PRrs {}
///`reset()` method sets P1CPPM0PR to value 0
impl crate::Resettable for P1CPPM0PRrs {}
